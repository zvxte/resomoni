use crate::error::Error;

type Result<T> = std::result::Result<T, Error>;

pub struct Processor {
    total: Option<u64>,
    idle: Option<u64>,
}

impl Processor {
    pub fn new(total: Option<u64>, idle: Option<u64>) -> Self {
        Self { total, idle }
    }

    pub fn update(&mut self, total: Option<u64>, idle: Option<u64>) {
        self.total = total;
        self.idle = idle;
    }

    pub fn calculate(&self, total: Option<u64>, idle: Option<u64>) -> Result<u8> {
        let last_total = match self.total {
            Some(total) => total,
            None => return Err(Error::EmptyValueError),
        };
        let last_idle = match self.idle {
            Some(idle) => idle,
            None => return Err(Error::EmptyValueError),
        };
        let current_total = match total {
            Some(total) => total,
            None => return Err(Error::EmptyValueError),
        };
        let current_idle = match idle {
            Some(idle) => idle,
            None => return Err(Error::EmptyValueError),
        };

        if current_total < last_total || current_idle < last_idle {
            return Err(Error::InvalidValueError);
        }

        let total_difference = (current_total - last_total) as f32;
        let idle_difference = (current_idle - last_idle) as f32;

        let usage = 100.0 * (1.0 - (idle_difference / total_difference));
        if usage > 100.0 || usage < 0.0 {
            return Err(Error::InvalidValueError);
        }

        Ok(usage as u8)
    }
}

pub fn parse_proc_stat_line(line: &str) -> Result<[u64; 10]> {
    let mut numbers = [0; 10];
    for (idx, number) in line.trim().split_whitespace().skip(1).enumerate() {
        let number: u64 = match number.parse() {
            Ok(number) => number,
            Err(_) => return Err(Error::ParseValueError),
        };
        numbers[idx] = number;
    }
    Ok(numbers)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn processor_new() {
        let processor = Processor::new(Some(10), None);
        assert_eq!(processor.total, Some(10));
        assert_eq!(processor.idle, None);

        let processor = Processor::new(None, Some(10));
        assert_eq!(processor.total, None);
        assert_eq!(processor.idle, Some(10));
    }

    #[test]
    fn processor_update() {
        let mut processor = Processor::new(Some(2), None);
        processor.update(Some(4), Some(8));
        assert_eq!(processor.total, Some(4));
        assert_eq!(processor.idle, Some(8));
    }

    #[test]
    fn processor_calculate_valid() {
        let processor = Processor::new(Some(150), Some(140));
        let result = processor.calculate(Some(160), Some(142)).unwrap();
        assert_eq!(result, 80);
    }

    #[test]
    fn processor_calculate_invalid() {
        let processor = Processor::new(Some(150), Some(160));
        let result = processor.calculate(None, Some(170));
        assert!(result.is_err());

        let processor = Processor::new(Some(150), Some(140));
        let result = processor.calculate(Some(151), Some(145));
        assert!(result.is_err());
    }

    #[test]
    fn parse_valid() {
        let line = "cpu  1681578 19 113823 22203616 1068 22446 11809 0 0 0";
        let numbers: [u64; 10] = parse_proc_stat_line(line).unwrap();
        let expected_numbers: [u64; 10] =
            [1681578, 19, 113823, 22203616, 1068, 22446, 11809, 0, 0, 0];
        assert_eq!(numbers, expected_numbers);
    }

    #[test]
    #[should_panic]
    fn parse_invalid() {
        let line = "1681578 19 113823 22203616 1068 22446 11809 0 0 0";
        let _numbers: [u64; 10] = parse_proc_stat_line(line).unwrap();

        let line = "cpu  19 113823 22203616 1068 22446 11809 0 0 0";
        let _numbers: [u64; 10] = parse_proc_stat_line(line).unwrap();

        let line = "cpu  1681578 19 _ 22203616 1068 22446 11809 0 0 0";
        let _numbers: [u64; 10] = parse_proc_stat_line(line).unwrap();
    }
}
