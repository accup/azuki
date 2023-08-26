use azuki::core::splitter::{Eighth, Full, Half, Quarter};

fn main() {
    let data = (0..13).map(|v| (v * 8 + 1) as u8).collect::<Vec<_>>();
    println!("   data: {:x?}", data);

    let full = Full::unroll(&data);
    let half = Half::unroll(&data);
    let quarter = Quarter::unroll(&data);
    let eighth = Eighth::unroll(&data);
    println!(
        "   full: {}",
        full.iter()
            .map(|v| format!("{:08b}", v))
            .collect::<Vec<_>>()
            .join("")
    );
    println!(
        "   half: {}",
        half.iter()
            .map(|v| format!("{:04b}", v))
            .collect::<Vec<_>>()
            .join("")
    );
    println!(
        "quarter: {}",
        quarter
            .iter()
            .map(|v| format!("{:02b}", v))
            .collect::<Vec<_>>()
            .join("")
    );
    println!(
        " eighth: {}",
        eighth
            .iter()
            .map(|v| format!("{:01b}", v))
            .collect::<Vec<_>>()
            .join("")
    );

    let full = Full::roll(&full);
    let half = Half::roll(&half);
    let quarter = Quarter::roll(&quarter);
    let eighth = Eighth::roll(&eighth);

    println!("   full: {:x?}", full);
    println!("   half: {:x?}", half);
    println!("quarter: {:x?}", quarter);
    println!(" eighth: {:x?}", eighth);
}
