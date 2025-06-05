use compose_idents::compose_idents;

macro_rules! create_static {
    () => {
        compose_idents!(
            MY_UNIQUE_STATIC = hash(1),
            MY_OTHER_UNIQUE_STATIC = hash(2),
            {
                static MY_UNIQUE_STATIC: u32 = 42;
                static MY_OTHER_UNIQUE_STATIC: u32 = 42;
            }
        );
    };
}

create_static!();
create_static!();
