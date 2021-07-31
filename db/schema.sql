CREATE FUNCTION public.decrement_tickets_sold() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
DECLARE
    updated integer;
BEGIN
    UPDATE
        ticket_types
    SET
        sold = sold - 1
    WHERE
        ticket_types.id = OLD.ticket_type_id;

	GET DIAGNOSTICS updated = ROW_COUNT;

    RETURN NULL;
END;
$$;

CREATE FUNCTION public.increment_tickets_sold() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
DECLARE
    updated integer;
BEGIN
    UPDATE
        ticket_types
    SET
        sold = sold + 1
    WHERE
        ticket_types.id = NEW.ticket_type_id AND ticket_types.sold < ticket_types.allocation;

	GET DIAGNOSTICS updated = ROW_COUNT;

	IF updated = 0 THEN
        RAISE EXCEPTION 'Sorry, no tickets left for tty %', NEW.ticket_type_id;
    END IF;
    RETURN NEW;
END;
$$;


CREATE FUNCTION public.set_ticket_price() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
DECLARE
    new_price_cents integer;
BEGIN
    SELECT price_cents INTO new_price_cents FROM pricing
	INNER JOIN ticket_types on ticket_types.id = pricing.ticket_type_id
    WHERE
        pricing.ticket_type_id = NEW.ticket_type_id 
		AND ticket_types.sold >= pricing.from_allocation
	ORDER BY pricing.from_allocation DESC NULLS LAST
	LIMIT 1;

	NEW.price_cents = new_price_cents;

    RETURN NEW;
END;
$$;


CREATE TABLE public.buyers (
    id bigint NOT NULL,
    name character varying NOT NULL
);

CREATE SEQUENCE public.buyers_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;

ALTER SEQUENCE public.buyers_id_seq OWNED BY public.buyers.id;


CREATE TABLE public.pricing (
    id integer NOT NULL,
    ticket_type_id integer NOT NULL,
    from_allocation integer NOT NULL,
    price_cents integer
);


CREATE SEQUENCE public.pricing_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.pricing_id_seq OWNED BY public.pricing.id;

CREATE TABLE public.ticket_types (
    id integer NOT NULL,
    name character varying NOT NULL,
    allocation integer DEFAULT 0 NOT NULL,
    sold integer DEFAULT 0 NOT NULL
);


CREATE SEQUENCE public.ticket_types_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.ticket_types_id_seq OWNED BY public.ticket_types.id;

CREATE TABLE public.tickets (
    id bigint NOT NULL,
    buyer_id bigint NOT NULL,
    ticket_type_id integer NOT NULL,
    price_cents integer DEFAULT 0 NOT NULL
);

CREATE SEQUENCE public.tickets_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;

ALTER SEQUENCE public.tickets_id_seq OWNED BY public.tickets.id;

ALTER TABLE ONLY public.buyers ALTER COLUMN id SET DEFAULT nextval('public.buyers_id_seq'::regclass);
ALTER TABLE ONLY public.pricing ALTER COLUMN id SET DEFAULT nextval('public.pricing_id_seq'::regclass);
ALTER TABLE ONLY public.ticket_types ALTER COLUMN id SET DEFAULT nextval('public.ticket_types_id_seq'::regclass);
ALTER TABLE ONLY public.tickets ALTER COLUMN id SET DEFAULT nextval('public.tickets_id_seq'::regclass);

ALTER TABLE ONLY public.buyers
    ADD CONSTRAINT buyers_pkey PRIMARY KEY (id);

ALTER TABLE ONLY public.pricing
    ADD CONSTRAINT pricing_pkey PRIMARY KEY (id);

ALTER TABLE ONLY public.ticket_types
    ADD CONSTRAINT ticket_types_pkey PRIMARY KEY (id);

ALTER TABLE ONLY public.tickets
    ADD CONSTRAINT tickets_pkey PRIMARY KEY (id);

ALTER TABLE ONLY public.pricing
    ADD CONSTRAINT uq_from_allocation UNIQUE (ticket_type_id, from_allocation) INCLUDE (price_cents);

ALTER TABLE ONLY public.buyers
    ADD CONSTRAINT uq_name UNIQUE (name);

CREATE INDEX fki_fk_buyer ON public.tickets USING btree (buyer_id);

CREATE INDEX fki_fk_ticket_type ON public.tickets USING btree (ticket_type_id);

CREATE TRIGGER ticket_set_ticket_price BEFORE INSERT ON public.tickets FOR EACH ROW EXECUTE FUNCTION public.set_ticket_price();

CREATE TRIGGER tickets_decrement_sold AFTER DELETE ON public.tickets FOR EACH ROW EXECUTE FUNCTION public.decrement_tickets_sold();

CREATE TRIGGER tickets_increment_sold BEFORE INSERT ON public.tickets FOR EACH ROW EXECUTE FUNCTION public.increment_tickets_sold();

ALTER TABLE ONLY public.tickets
    ADD CONSTRAINT fk_buyer FOREIGN KEY (buyer_id) REFERENCES public.buyers(id) ON UPDATE CASCADE ON DELETE RESTRICT NOT VALID;

ALTER TABLE ONLY public.tickets
    ADD CONSTRAINT fk_ticket_type FOREIGN KEY (ticket_type_id) REFERENCES public.ticket_types(id) NOT VALID;

ALTER TABLE ONLY public.pricing
    ADD CONSTRAINT fk_ticket_type FOREIGN KEY (ticket_type_id) REFERENCES public.ticket_types(id);
