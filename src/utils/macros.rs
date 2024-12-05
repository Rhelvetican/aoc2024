#[macro_export]
macro_rules! map_solution {
    ($cli:ident : $($day:expr => $sol:expr,)+ $(,)?) => {
        match $cli .day {
            $($day => {
                let sol = $sol;
                let input = sol.get_input($cli.input.as_deref())?;
                println!("\tp1: {}\n\tp2: {}", sol.part_one(&input)?, sol.part_two(&input)?);
            },)+
            _ => return Err(Error::UnsupportedDay),
        }
    };
}
