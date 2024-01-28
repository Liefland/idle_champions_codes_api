create or replace function get_random_source_id()
    returns integer as $rand_src_id$
declare
    rand_src_id integer;
begin
    select 1 + floor(random() * (select count(*) as sources_length from sources))::int
    into rand_src_id;
    return rand_src_id;
end
$rand_src_id$ language plpgsql;

create or replace function insert_random_code(author_source_id integer, len integer)
    returns void as $insert_random_code$
begin
    insert into codes (code, expires_at, submitter_id, creator_id, lister_id)
    values (
       CONCAT('MOCK', upper(substr(md5(random()::text), 0, len - 4))::text),
       CURRENT_DATE + INTERVAL '1 day' * ((-60 + (180 * random()))::int),
       get_random_source_id(),
       get_random_source_id(),
       author_source_id
   );
end
$insert_random_code$ language plpgsql;

insert into sources (name, url)
    values ('foo', 'https://foo.com'),
           ('bar', 'https://bar.com'),
           ('baz', 'https://baz.com'),
           ('qux', 'https://qux.com'),
           ('author', 'https://apiauthor.com'),
           ('common_code_poster', 'https://idle-codes.com');

insert into api_keys (source_id, api_key)
    values ((select id from sources where name = 'author'), 'common_api_key');

do $$
declare author_source_id integer;
begin
    select id from sources where name = 'author' into author_source_id;

    insert into codes (code, expires_at, submitter_id, creator_id, lister_id) values
        ('EXPI-REDC-ODES', CURRENT_DATE + INTERVAL '-1 week', get_random_source_id(), get_random_source_id(), author_source_id),
        ('WELC-OMEM-FOO!', CURRENT_DATE + INTERVAL '4 weeks', get_random_source_id(), get_random_source_id(), author_source_id),
        ('WELC-OMEM-BAR!', CURRENT_DATE + INTERVAL '4 weeks', get_random_source_id(), get_random_source_id(), author_source_id),
        ('WELC-OMEM-BAZ!', CURRENT_DATE + INTERVAL '4 weeks', get_random_source_id(), get_random_source_id(), author_source_id),
        ('WELC-OMEM-QUX!', CURRENT_DATE + INTERVAL '4 weeks', get_random_source_id(), get_random_source_id(), author_source_id),
        ('ASTR-ALEL-FEME-RGE!', CURRENT_DATE + INTERVAL '4 weeks', get_random_source_id(), get_random_source_id(), author_source_id),
        ('NEWA-CCOU-NTNE-WME!', CURRENT_DATE + INTERVAL '4 weeks', get_random_source_id(), get_random_source_id(), author_source_id),
        ('WYLL-YWON-KA!!', CURRENT_DATE + INTERVAL '4 weeks', get_random_source_id(), get_random_source_id(), author_source_id)
    ;

    for i in 1..10 loop
        perform insert_random_code(author_source_id, 12);
        perform insert_random_code(author_source_id, 16);
    end loop;
end $$;
