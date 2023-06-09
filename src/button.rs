#[derive(Debug)]
pub enum Button {
    //Main face of remote
    Home,
    Menu,
    ArrowUp,
    ArrowLeft,
    ArrowRight,
    ArrowDown,
    Ok,
    Exit,
    Hamburger,
    Play,
    Pause,
    Rewind,
    FastForward,
    Record,
    Stop,
    Yellow,
    Blue,
    Red,
    Green,
    Num1,
    Num2,
    Num3,
    Num4,
    Num5,
    Num6,
    Num7,
    Num8,
    Num9,
    Num0,
    NumEnter,
    NumMinus,

    //Left side
    Input,
    ChannelDown,
    ChannelUp,
    ChannelPrevious,
    Eject,

    //Right side
    Power,
    VolumeUp,
    VolumeDown,
    VolumeMute,

}

impl TryFrom<u8> for Button {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            121 => Ok(Self::Home),
            26 => Ok(Self::Menu),
            96 => Ok(Self::ArrowUp),
            101 => Ok(Self::ArrowLeft),
            97 => Ok(Self::ArrowDown),
            98 => Ok(Self::ArrowRight),
            104 => Ok(Self::Ok),
            88 => Ok(Self::Exit),
            31 => Ok(Self::Hamburger),
            69 => Ok(Self::Rewind),
            71 => Ok(Self::Play),
            72 => Ok(Self::FastForward),
            73 => Ok(Self::Record),
            70 => Ok(Self::Stop),
            74 => Ok(Self::Pause),
            21 => Ok(Self::Yellow),
            22 => Ok(Self::Blue),
            108 => Ok(Self::Red),
            20 => Ok(Self::Green),
            4 => Ok(Self::Num1),
            5 => Ok(Self::Num2),
            6 => Ok(Self::Num3),
            8 => Ok(Self::Num4),
            9 => Ok(Self::Num5),
            10 => Ok(Self::Num6),
            12 => Ok(Self::Num7),
            13 => Ok(Self::Num8),
            14 => Ok(Self::Num9),
            35 => Ok(Self::NumMinus),
            17 => Ok(Self::Num0),
            104 => Ok(Self::NumEnter),

            1 => Ok(Self::Input),
            18 => Ok(Self::ChannelUp),
            16 => Ok(Self::ChannelDown),
            19 => Ok(Self::ChannelPrevious),
            107 => Ok(Self::Eject),

            2 => Ok(Self::Power),
            7 => Ok(Self::VolumeUp),
            11 => Ok(Self::VolumeDown),
            15 => Ok(Self::VolumeMute),

            _ => Err(())

        }
    }
}