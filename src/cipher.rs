use std::fmt::Display;

pub(crate) const SIZE: usize = 192;

#[derive(Clone, Copy)]
pub(crate) struct VignDict(pub(crate) [char; SIZE]);

//creates and wraps a new dictionary for the cipher
impl VignDict{
    pub(crate) fn new() -> VignDict{
        let mut dict = r##" !"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\]^_`abcdefghijklmnopqrstuvwxyz{|}~ ¡¢£¤¥¦§¨©ª«¬­®¯°±²³´µ¶·¸¹º»¼½¾¿ÀÁÂÃÄÅÆÇÈÉÊËÌÍÎÏÐÑÒÓÔÕÖ×ØÙÚÛÜÝÞßàáâãäåæçèéêëìíîïðñòóôõö÷øùúûüýþÿ"##.to_string();
        dict.push('\n');
        dict.push('\r');
        let mut dict_char_array = [' ';SIZE];
        for(idx, ch) in dict.chars().enumerate(){
            dict_char_array[idx] = ch;
        }
        return VignDict(dict_char_array);
    }
    
    pub(crate) fn get_string(&self) -> String{
        let mut s = String::new();
        for ch in self.0 {
            s.push(ch);
        }
        s
    }
}


pub struct Hello{
    name: String,
}

pub fn new_hello() -> Hello{
    Hello{
        name: "pj".to_string(),
    }
}
impl Display for Hello{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
