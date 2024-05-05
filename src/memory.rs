use crate::error::Error;

type Result<T> = std::result::Result<T, Error>;

pub struct Memory {
    total: Option<u64>,
    available: Option<u64>,
}

impl Memory {
    pub fn new(total: Option<u64>, available: Option<u64>) -> Self {
        Self { total, available }
    }

    pub fn update(&mut self, total: Option<u64>, available: Option<u64>) {
        self.total = total;
        self.available = available;
    }

    pub fn calculate(&self) -> Result<u8> {
        let total = match self.total {
            Some(total) => total as f32,
            None => return Err(Error::EmptyValueError),
        };
        let available = match self.available {
            Some(available) => available as f32,
            None => return Err(Error::EmptyValueError),
        };

        if available > total {
            return Err(Error::InvalidValueError);
        }

        let usage = 100.0 * (1.0 - (available / total));
        Ok(usage as u8)
    }
}

pub fn parse_mem_stat_line(line: &str) -> Result<u64> {
    match line.trim().split_whitespace().nth(1) {
        Some(value) => match value.parse::<u64>() {
            Ok(value) => Ok(value),
            Err(_) => Err(Error::ParseValueError),
        },
        None => Err(Error::EmptyValueError),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn memory_new() {
        let memory = Memory::new(Some(10), None);
        assert_eq!(memory.total, Some(10));
        assert_eq!(memory.available, None);
    }

    #[test]
    fn memory_update() {
        let mut memory = Memory::new(Some(2), None);
        memory.update(Some(4), Some(8));
        assert_eq!(memory.total, Some(4));
        assert_eq!(memory.available, Some(8));
    }

    #[test]
    fn memory_calculate_valid() {
        let memory = Memory::new(Some(200), Some(100));
        let result = memory.calculate().unwrap();
        assert_eq!(result, 50);
    }

    #[test]
    fn memory_calculate_invalid() {
        let memory = Memory::new(Some(150), Some(160));
        let result = memory.calculate();
        assert!(result.is_err());

        let memory = Memory::new(Some(150), None);
        let result = memory.calculate();
        assert!(result.is_err());
    }

    #[test]
    fn parse_valid() {
        let line = "MemTotal:       16215968 kB";
        let result = parse_mem_stat_line(line).unwrap();
        assert_eq!(result, 16215968);
    }

    #[test]
    #[should_panic]
    fn parse_invalid() {
        let line = "16215968 kB";
        let _result = parse_mem_stat_line(line).unwrap();

        let line = "MemTotal:16215968 kB";
        let _result = parse_mem_stat_line(line).unwrap();

        let line = "MemTotal:       _ kB";
        let _result = parse_mem_stat_line(line).unwrap();
    }
}
