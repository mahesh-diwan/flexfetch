pub struct Logo {
    pub lines: &'static [&'static str],
    pub colors: &'static [&'static str],
}

pub fn render(logo: &Logo) -> Vec<String> {
    let target = 28;
    let mut out = Vec::with_capacity(target);
    for line in logo.lines {
        let mut s = line.to_string();
        if logo.colors.len() >= 3 {
            s = s.replace("${1}", logo.colors[0]);
            s = s.replace("${2}", logo.colors[1]);
            s = s.replace("${3}", logo.colors[2]);
        }
        out.push(s);
    }
    while out.len() < target {
        out.push(String::new());
    }
    out
}

pub fn logo_width(rendered: &[String]) -> usize {
    rendered.iter().map(|l| l.len()).max().unwrap_or(0)
}

pub fn detect(os_id: &str) -> &'static Logo {
    match os_id {
        "arch" | "cachyos" | "arcolinux" | "artix" => &ARCH_LOGO,
        "manjaro" => &MANJARO_LOGO,
        "endeavouros" => &ENDEAVOUROS_LOGO,
        "debian" | "raspbian" => &DEBIAN_LOGO,
        "ubuntu" | "linuxmint" | "pop" | "elementary" | "zorin" => &UBUNTU_LOGO,
        "fedora" => &FEDORA_LOGO,
        "nixos" => &NIXOS_LOGO,
        "gentoo" => &GENTOO_LOGO,
        "alpine" => &ALPINE_LOGO,
        "void" => &VOID_LOGO,
        "centos" => &CENTOS_LOGO,
        "opensuse" | "opensuse-leap" | "opensuse-tumbleweed" => &OPENSUSE_LOGO,
        "kali" => &KALI_LOGO,
        _ if cfg!(target_os = "macos") => &MACOS_LOGO,
        _ => &GENERIC_LOGO,
    }
}

const CYAN: &str = "\x1b[36m";
const RESET: &str = "\x1b[0m";
const WHITE: &str = "\x1b[37m";
const RED: &str = "\x1b[31m";
const GREEN: &str = "\x1b[32m";
const BLUE: &str = "\x1b[34m";
const MAGENTA: &str = "\x1b[35m";

const GENERIC_LOGO: Logo = Logo {
    lines: &[
        "  ___________",
        " /   _____/ ",
        " \\_____  \\  ",
        " /        \\ ",
        "/_______  /  ",
        "        \\/   ",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
    ],
    colors: &[CYAN, RESET, WHITE],
};

const ARCH_LOGO: Logo = Logo {
    lines: &[
        "      ${1}/\\      ",
        "     ${1}/  \\${2}     ",
        "    ${1}/ /\\ \\${2}    ",
        "   ${1}/ ____ \\${2}   ",
        "  ${1}/_/    \\_\\${2}  ",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
    ],
    colors: &[CYAN, RESET, WHITE],
};

const MANJARO_LOGO: Logo = Logo {
    lines: &[
        " ${1}███████╗${2}██╗  ${1}██╗${2}",
        " ${1}██╔════╝${2}╚██╗${1}██╔╝${2}",
        " ${1}███████╗${2} ╚███╔╝${2} ",
        " ${1}╚════██║${2} ██╔██╗${2} ",
        " ${1}███████║${2}██╔╝ ${1}██╗${2}",
        " ${1}╚══════╝${2}╚═╝  ${1}╚═╝${2}",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
    ],
    colors: &[GREEN, RESET, WHITE],
};

const ENDEAVOUROS_LOGO: Logo = Logo {
    lines: &[
        "        ${1}é${2}        ",
        "       ${1}/|\\${2}       ",
        "      ${1}/ | \\${2}      ",
        "     ${1}/  |  \\${2}     ",
        "    ${1}/   |   \\${2}    ",
        "   ${1}/    |    \\${2}   ",
        "  ${1}/     |     \\${2}  ",
        " ${1}/      |      \\${2} ",
        "${1}/_______|_______\\${2}",
        "  ${1}${3}              ${2}  ",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
    ],
    colors: &[CYAN, RESET, WHITE],
};

const DEBIAN_LOGO: Logo = Logo {
    lines: &[
        "  ${1}_______${2}",
        " ${1}|  ___  |${2}",
        " ${1}| |   | |${2}",
        " ${1}| |___| |${2}",
        " ${1}|_______|${2}",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
    ],
    colors: &[RED, RESET, WHITE],
};

const UBUNTU_LOGO: Logo = Logo {
    lines: &[
        "  ${1}_   _   ${2}",
        " ${1}| | | |  ${2}",
        " ${1}| |_| |  ${2}",
        " ${1}|  _  |  ${2}",
        " ${1}|_| |_|  ${2}",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
    ],
    colors: &[RED, RESET, WHITE],
};

const FEDORA_LOGO: Logo = Logo {
    lines: &[
        "  ${1}________${2}",
        " ${1}/   __   \\${2}",
        "${1}|   /  \\  |${2}",
        "${1}|  |   |  |${2}",
        " ${1}\\  \\_/  /${2}",
        "  ${1}\\_____/${2}",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
    ],
    colors: &[BLUE, RESET, WHITE],
};

const NIXOS_LOGO: Logo = Logo {
    lines: &[
        "  ${1}~~~~~~~${2}",
        " ${1}:::::::: ${2}",
        " ${1}:::::::: ${2}",
        " ${1}:::::::: ${2}",
        "  ${1}::::::  ${2}",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
    ],
    colors: &[CYAN, RESET, WHITE],
};

const GENTOO_LOGO: Logo = Logo {
    lines: &[
        "      ${1}.--.  ${2}",
        "     ${1}|o_o |${2} ",
        "     ${1}|:_/ |${2} ",
        "    ${1}//   \\ \\${2}",
        "   ${1}(|     | )${2}",
        "  ${1}/'\\_   _/\\`\\${2}",
        "  ${1}\\___)=(___/${2}",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
    ],
    colors: &[MAGENTA, RESET, WHITE],
};

const ALPINE_LOGO: Logo = Logo {
    lines: &[
        "        ${1} ${2}      ",
        "       ${1}/ \\${2}     ",
        "      ${1}/   \\${2}    ",
        "     ${1}/ /\\  \\${2}   ",
        "    ${1}/ /  \\  \\${2}  ",
        "   ${1}/ /    \\  \\${2} ",
        "  ${1}/ /      \\  \\${2}",
        " ${1}/ /________\\  \\${2}",
        " ${1}\\(___________)${2}",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
    ],
    colors: &[CYAN, RESET, WHITE],
};

const VOID_LOGO: Logo = Logo {
    lines: &[
        "  ${1}_____._._${2}",
        "  ${1}\\____| |/${2}",
        "  ${1}/     | ${2}",
        " ${1}/  _   | ${2}",
        "${1}|  (_)  |${2}",
        " ${1}\\_____/ ${2}",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
    ],
    colors: &[MAGENTA, RESET, WHITE],
};

const CENTOS_LOGO: Logo = Logo {
    lines: &[
        "  ${1}________${2}",
        " ${1}/   __   \\${2}",
        "${1}|   /  \\  |${2}",
        "${1}|  |   |  |${2}",
        " ${1}\\  \\_/  /${2}",
        "  ${1}\\_____/${2}",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
    ],
    colors: &[RED, RESET, WHITE],
};

const OPENSUSE_LOGO: Logo = Logo {
    lines: &[
        "   ${1}_______  ${2}",
        "  ${1}/  ___  \\ ${2}",
        " ${1}/  /   \\  \\${2}",
        " ${1}|  |     |${2}",
        " ${1}\\  \\___/  /${2}",
        "  ${1}\\_______/ ${2}",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
    ],
    colors: &[GREEN, RESET, WHITE],
};

const KALI_LOGO: Logo = Logo {
    lines: &[
        "        ${1}${3}    ${2}  ",
        "       ${1}${3}  ${2}     ",
        "      ${1}${3}    ${2}    ",
        "     ${1}${3}  ${2}       ",
        "    ${1}${3}    ${2}      ",
        "   ${1}${3}  ${2}         ",
        "  ${1}${3}    ${2}        ",
        " ${1}${3}  ${2}           ",
        "${1}${3}  ${2}            ",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
    ],
    colors: &[BLUE, RESET, WHITE],
};

const MACOS_LOGO: Logo = Logo {
    lines: &[
        "  ${1}.::::.${2}",
        " ${1}:::::::${2}",
        " ${1}:::::::${2}",
        " ${1}:::::::${2}",
        " ${1}:::::::${2}",
        "  ${1}'::::'${2}",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
    ],
    colors: &[WHITE, RESET, CYAN],
};
