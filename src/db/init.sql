CREATE USER "generate_admin" WITH PASSWORD 'generate_tech_app';

CREATE TABLE applicants (
    nuid varchar PRIMARY KEY,
    applicant_name varchar NOT NULL,
    registration_time timestamp with time zone NOT NULL,
    token uuid UNIQUE NOT NULL
);

CREATE TABLE solutions (
    solution_id serial PRIMARY KEY,
    challenge_string varchar NOT NULL,
    solution json NOT NULL,
    token uuid NOT NULL REFERENCES applicants (token)
);

CREATE TABLE submissions (
    submission_id serial PRIMARY KEY,
    solution_id integer NOT NULL REFERENCES solutions (solution_id),
    token uuid NOT NULL REFERENCES applicants (token),
    ok boolean NOT NULL,
    submission_time timestamp with time zone NOT NULL
);

