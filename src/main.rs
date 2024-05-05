use resomoni::{
    error::{Error, File},
    memory, processor,
};
use std::io::{BufRead, BufReader, Write};

type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let mut processor = processor::Processor::new(None, None);
    let mut memory = memory::Memory::new(None, None);
    println!("  CPU   MEM");

    loop {
        /* ----------------- */
        /* --- CPU USAGE --- */
        /* ----------------- */

        // Opening file with processor stats
        let proc_file = match std::fs::File::open("/proc/stat") {
            Ok(file) => file,
            Err(_) => return Err(Error::FileOpenError(File::ProcessorFile)),
        };

        let mut proc_buf_reader = BufReader::new(proc_file);
        let mut proc_line = String::new();
        if let Err(_) = proc_buf_reader.read_line(&mut proc_line) {
            return Err(Error::ReadError(File::ProcessorFile));
        }

        // Parsing needed data from processor stats file (first line)
        let numbers = match processor::parse_proc_stat_line(&proc_line) {
            Ok(numbers) => numbers,
            Err(err) => return Err(err),
        };
        let total: u64 = numbers.iter().sum();
        let idle = numbers[3];

        // Calculating processor usage starting from last iteration to now
        match processor.calculate(Some(total), Some(idle)) {
            Ok(proc_usage) => {
                let formatted = format!("{:>4}%", proc_usage);
                print!("{}", formatted);
            }
            Err(_) => print!("{:>5}", '-'),
        }
        processor.update(Some(total), Some(idle));

        /* ----------------- */
        /* --- MEM USAGE --- */
        /* ----------------- */

        // Opening file with memory stats
        let mem_file = match std::fs::File::open("/proc/meminfo") {
            Ok(file) => file,
            Err(_) => return Err(Error::FileOpenError(File::MemoryFile)),
        };

        let mem_buf_reader = BufReader::new(mem_file);
        let mut lines = mem_buf_reader.lines();

        // Parsing needed data from memory stats file (first and third line)
        let mem_total_line = match lines.nth(0) {
            Some(line) => match line {
                Ok(line) => line,
                Err(_) => return Err(Error::ReadError(File::MemoryFile)),
            },
            None => return Err(Error::ReadError(File::MemoryFile)),
        };
        // Third line is now second since `nth()` method consumes the iterator
        let mem_available_line = match lines.nth(1) {
            Some(line) => match line {
                Ok(line) => line,
                Err(_) => return Err(Error::ReadError(File::MemoryFile)),
            },
            None => return Err(Error::ReadError(File::MemoryFile)),
        };

        let mem_total = match memory::parse_mem_stat_line(&mem_total_line) {
            Ok(mem_total) => mem_total,
            Err(err) => return Err(err),
        };
        let mem_available = match memory::parse_mem_stat_line(&mem_available_line) {
            Ok(mem_available) => mem_available,
            Err(err) => return Err(err),
        };
        memory.update(Some(mem_total), Some(mem_available));

        // Calculating current memory usage
        match memory.calculate() {
            Ok(mem_usage) => {
                let formatted = format!("{:>5}%", mem_usage);
                print!("{}", formatted);
            }
            Err(_) => print!("{:>5}", '-'),
        }

        /* ----------------- */

        // Flushing at the end to update both processor and memory usages
        if let Err(_) = std::io::stdout().flush() {
            return Err(Error::OutputFlushError);
        }

        // Updating the console output every 2 seconds
        std::thread::sleep(core::time::Duration::from_secs(2));

        /*
            Using ANSI escape sequence to update the output in-place
            instead of repeatedly printing to a next line

            `\x1b` - Escape character
            `[2K`  - Erase the entire line
            `\r`   - Carriage return
        */
        print!("\x1b[2K\r");
    }
}
