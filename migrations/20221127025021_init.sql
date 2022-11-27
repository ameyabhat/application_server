-- Add migration script here
CREATE USER "generate_admin" WITH PASSWORD 'generate_tech_app';

CREATE TABLE IF NOT EXISTS applicants (
    nuid varchar PRIMARY KEY,
    applicant_name varchar NOT NULL,
    registration_time timestamp with time zone NOT NULL,
    token uuid UNIQUE NOT NULL,
    challenge_string varchar NOT NULL,
    solution json NOT NULL
);

CREATE TABLE IF NOT EXISTS submissions (
    submission_id serial PRIMARY KEY,
    -- solution_id integer NOT NULL REFERENCES problems (solution_id),
    nuid varchar NOT NULL REFERENCES applicants (nuid),
    ok boolean NOT NULL,
    submission_time timestamp with time zone NOT NULL
);


/*
 Right now this could be part of the applicants table
 I'm keeping it separate to eventually enable giving back new challenge
 string at each get_challenge string req
 Eventually get_challenge_string will generate its own challenge string, 
 write it to the db with a token, then we'll look up and verify solutions
 with the token
 */
/*
CREATE TABLE problems (
 solution_id serial PRIMARY KEY,
 challenge_string varchar NOT NULL,
 solution json NOT NULL,
 token uuid NOT NULL REFERENCES applicants (token)
);
 */
