trait NotEnoughString {
    fn split_once_with_index(&self, str: &str) -> (Option<(&str, &str)>, Option<usize>);
}
impl NotEnoughString for String {
    fn split_once_with_index(&self, str: &str) -> (Option<(&str, &str)>, Option<usize>) {
        let find = self.find(str);
        match find {
            Some(i) => (self.split_once(str), Some(i)),
            None => (None, None),
        }
    }
}
