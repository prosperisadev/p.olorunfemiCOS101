--
-- PostgreSQL database dump
--

-- Dumped from database version 17.2
-- Dumped by pg_dump version 17.2

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET transaction_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: customer_table; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.customer_table (
    c_id integer NOT NULL,
    c_name text NOT NULL,
    c_age integer NOT NULL,
    c_email text NOT NULL,
    c_mobile character varying(20) NOT NULL,
    e_id integer NOT NULL,
    data_id integer NOT NULL
);


ALTER TABLE public.customer_table OWNER TO postgres;

--
-- Name: dataplan_table; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.dataplan_table (
    data_id integer NOT NULL,
    data_size character(15) NOT NULL,
    data_duration integer NOT NULL,
    data_price_naira integer NOT NULL
);


ALTER TABLE public.dataplan_table OWNER TO postgres;

--
-- Name: department; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.department (
    dept_managerid integer NOT NULL,
    d_no integer NOT NULL,
    d_name character(30) NOT NULL,
    d_location character(30) NOT NULL,
    p_no integer NOT NULL
);


ALTER TABLE public.department OWNER TO postgres;

--
-- Name: project; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.project (
    p_no integer NOT NULL,
    p_name character(10) NOT NULL,
    p_duration character(30) NOT NULL,
    project_managerid integer NOT NULL
);


ALTER TABLE public.project OWNER TO postgres;

--
-- Name: staff; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.staff (
    staff_id integer NOT NULL,
    staff_name character(30) NOT NULL,
    d_no integer NOT NULL,
    staff_sal real NOT NULL,
    age integer NOT NULL,
    mobile character varying(15) NOT NULL
);


ALTER TABLE public.staff OWNER TO postgres;

--
-- Data for Name: customer_table; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.customer_table (c_id, c_name, c_age, c_email, c_mobile, e_id, data_id) FROM stdin;
110	Musta Karim	35	m_karim@gmail.com	08055089112	102	5
111	Lilian Jaiya	43	i_ijaiye@gmail.com	08055185341	100	3
112	Arthur Musa	50	a_mus@gmail.com	07055282813	107	10
113	Philip Akonjo	41	p_akonjo@gmail.com	09052356772	100	2
114	Marylene Mapa	33	m_mapa@gmail.com	08053333551	120	5
115	Oghenero Agor	50	o_agor@gmail.com	07055566774	117	11
116	Adams Bree	33	a_bree@gmail.com	08056765424	102	1
117	Okafor Matthias	45	o_matthias@gmail.com	08056763367	120	10
118	Samson Adeleke	65	s_adeleke@gmail.com	07056774423	117	11
119	Lawal Tamire	35	i_tamire@gmail.com	09052111101	107	5
120	James Job	44	j_job@gmail.com	08059693919	100	8
121	Matthew Jakande	21	m_jakande@gmail.com	07051232144	120	2
122	Jimila Adegboye	20	j_adegboye@gmail.com	08054921923	107	5
\.


--
-- Data for Name: dataplan_table; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.dataplan_table (data_id, data_size, data_duration, data_price_naira) FROM stdin;
1	350MB          	2	200
2	1.8GB          	14	500
3	3.9GB          	30	1000
4	7.5GB          	30	1500
5	9.2GB          	30	2000
6	10.8GB         	30	2500
7	14GB           	30	3000
8	18GB           	30	4000
9	24GB           	30	5000
10	29.9GB         	30	8000
11	50GB           	30	10000
\.


--
-- Data for Name: department; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.department (dept_managerid, d_no, d_name, d_location, p_no) FROM stdin;
108	1	Administration                	Ikeja                         	44
101	2	Account                       	Egbeda                        	11
100	3	Packaging                     	Ajah                          	44
120	4	Research                      	V.I                           	33
97	5	Account                       	Magodo                        	22
122	6	Operations                    	Mile 2                        	44
107	7	Packaging                     	Ketu                          	55
\.


--
-- Data for Name: project; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.project (p_no, p_name, p_duration, project_managerid) FROM stdin;
11	A         	9 Months                      	102
22	B         	14 Months                     	97
33	C         	16 Months                     	120
44	D         	25 Months                     	108
55	E         	9 Months                      	107
\.


--
-- Data for Name: staff; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.staff (staff_id, staff_name, d_no, staff_sal, age, mobile) FROM stdin;
101	Alade Joy                     	2	250000	33	8023089832
100	Mustapha Ali                  	3	175000	32	08063285831
107	Alokwe Martin                 	7	380000	48	07090082812
97	Dankade Aminat                	5	550000	40	09023688832
108	Josiah Joshua                 	1	120000	30	08053189131
102	Mankinde Mary                 	2	450000	55	09023487830
120	Adeleke Jane                  	4	200000	38	07061045862
122	Osahon Mark                   	6	320000	44	07030089811
104	Kuti Lawal                    	1	750000	35	09145689842
117	Suleman Ajayi                 	3	800000	50	7030089981
\.


--
-- Name: customer_table customer_table_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.customer_table
    ADD CONSTRAINT customer_table_pkey PRIMARY KEY (c_id);


--
-- Name: dataplan_table dataplan_table_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.dataplan_table
    ADD CONSTRAINT dataplan_table_pkey PRIMARY KEY (data_id);


--
-- Name: department department_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.department
    ADD CONSTRAINT department_pkey PRIMARY KEY (d_no);


--
-- Name: staff employees_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.staff
    ADD CONSTRAINT employees_pkey PRIMARY KEY (staff_id);


--
-- Name: project project_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.project
    ADD CONSTRAINT project_pkey PRIMARY KEY (p_no);


--
-- PostgreSQL database dump complete
--

