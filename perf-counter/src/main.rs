use windows::{core::*, Win32::System::Performance::*};

fn main() {
    unsafe {
        let mut query = 0;
        PdhOpenQueryW(None, 0, &mut query);

        let mut counter = 0;
        //let query_Str = r#"\\Processor(0)\\% Processor Time"#;
        let counter_path = "Processor(_Total)\\% Processor Time" + '\0';
        //let counter_path = counter_path.encode_utf16().collect::<Vec<u16>>();

        PdhAddCounterW(
            query,
            w!(counter_path),
            0,
            &mut counter,
        );

        loop {
            std::thread::sleep(std::time::Duration::new(1, 0));
            PdhCollectQueryData(query);

            let mut value = Default::default();
            if 0 == PdhGetFormattedCounterValue(counter, PDH_FMT_DOUBLE, None, &mut value) {
                println!("{:.2}", value.Anonymous.doubleValue);
            }
        }
    }
}