fn main() {
    let bkdata: [u8; 1000] = [
        32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32,
        32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32,
        32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32,
        32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32,
        142, 35, 239, 247, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32,
        32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 233, 38, 100, 32, 80,
        231, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32,
        32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 245, 160, 160, 11, 37, 160, 32, 32, 32,
        32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32,
        32, 32, 32, 32, 32, 32, 32, 32, 45, 160, 160, 141, 145, 245, 252, 98, 248, 247, 248, 98,
        32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 28,
        32, 32, 32, 32, 32, 32, 95, 160, 160, 141, 160, 160, 160, 160, 160, 160, 160, 160, 247,
        121, 98, 248, 247, 98, 121, 32, 32, 32, 99, 32, 214, 32, 32, 32, 62, 32, 163, 5, 29, 32,
        36, 142, 32, 32, 30, 32, 32, 124, 160, 160, 228, 249, 120, 160, 160, 160, 160, 160, 160,
        160, 105, 218, 38, 170, 95, 223, 32, 32, 58, 32, 32, 32, 32, 9, 53, 19, 28, 62, 9, 9, 149,
        9, 12, 53, 32, 74, 214, 32, 245, 160, 183, 90, 160, 160, 160, 160, 209, 239, 160, 160, 31,
        73, 193, 107, 62, 160, 32, 32, 32, 24, 181, 41, 30, 32, 0, 17, 178, 49, 37, 74, 48, 36,
        110, 62, 62, 211, 53, 73, 151, 160, 105, 32, 229, 160, 160, 160, 239, 31, 227, 160, 23,
        142, 6, 163, 0, 62, 163, 24, 163, 37, 28, 193, 53, 129, 145, 77, 77, 142, 36, 16, 23, 38,
        190, 77, 130, 28, 16, 11, 0, 160, 101, 32, 229, 160, 160, 231, 32, 226, 160, 160, 83, 160,
        160, 227, 117, 90, 139, 62, 52, 48, 128, 16, 90, 90, 218, 77, 151, 77, 77, 77, 77, 77, 77,
        0, 130, 130, 65, 31, 245, 160, 160, 247, 160, 160, 160, 160, 100, 122, 160, 160, 32, 205,
        83, 117, 141, 37, 117, 205, 90, 90, 90, 90, 90, 90, 218, 218, 218, 218, 218, 218, 218, 218,
        218, 218, 218, 218, 218, 32, 160, 160, 160, 160, 160, 160, 160, 160, 160, 160, 160, 160,
        193, 77, 218, 77, 77, 77, 77, 77, 206, 77, 77, 77, 77, 206, 218, 156, 176, 77, 193, 18,
        166, 188, 83, 128, 159, 206, 206, 160, 160, 160, 160, 160, 160, 160, 160, 160, 160, 160,
        160, 160, 76, 0, 77, 206, 203, 16, 201, 24, 141, 128, 23, 13, 144, 218, 134, 188, 128, 137,
        23, 137, 138, 190, 177, 164, 65, 19, 205, 156, 160, 160, 160, 160, 160, 160, 160, 160, 160,
        160, 160, 160, 177, 201, 13, 14, 165, 23, 131, 203, 5, 202, 95, 24, 138, 177, 133, 160,
        160, 160, 14, 160, 160, 134, 14, 160, 86, 92, 32, 20, 128, 79, 32, 74, 13, 160, 160, 160,
        160, 160, 160, 160, 86, 150, 160, 205, 158, 160, 137, 188, 14, 156, 160, 160, 160, 134,
        160, 14, 160, 160, 160, 160, 160, 147, 160, 160, 138, 101, 32, 32, 6, 32, 32, 32, 32, 32,
        20, 226, 160, 160, 160, 160, 160, 160, 160, 160, 160, 160, 160, 160, 160, 160, 160, 156,
        160, 160, 160, 160, 160, 160, 160, 160, 160, 160, 160, 160, 160, 117, 32, 32, 32, 32, 32,
        32, 32, 32, 32, 32, 254, 160, 160, 160, 160, 160, 160, 160, 160, 160, 160, 160, 160, 160,
        160, 160, 160, 160, 160, 160, 160, 160, 160, 160, 160, 160, 160, 160, 160, 231, 32, 32, 32,
        32, 32, 32, 32, 32, 32, 23, 160, 160, 160, 160, 160, 160, 160, 160, 160, 160, 160, 160,
        160, 160, 160, 160, 160, 160, 160, 160, 160, 160, 160, 160, 160, 160, 160, 160, 160, 160,
        223, 32, 32, 32, 32, 32, 32, 32, 32, 119, 2, 31, 31, 35, 160, 160, 160, 160, 160, 160, 160,
        160, 160, 160, 160, 160, 160, 160, 160, 160, 160, 160, 160, 160, 160, 160, 160, 160, 160,
        160, 160, 191, 123, 32, 28, 32, 32, 32, 32, 32, 32, 32, 82, 160, 160, 160, 160, 160, 160,
        160, 160, 160, 160, 160, 160, 160, 160, 160, 160, 160, 160, 160, 160, 160, 160, 160, 160,
        160, 160, 160, 160, 105, 32, 32, 32, 32, 32, 32, 32, 32, 32, 118, 160, 160, 160, 160, 160,
        160, 160, 160, 160, 160, 160, 160, 160, 160, 160, 160, 160, 160, 160, 160, 160, 160, 160,
        160, 160, 160, 160, 160, 126, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 225, 160, 160, 160,
        160, 160, 160, 160, 160, 160, 160, 160, 160, 160, 160, 160, 160, 160, 160, 160, 160, 160,
        160, 160, 160, 160, 160, 160, 99, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 100, 160, 236,
        160, 160, 160, 160, 160, 160, 160, 160, 160, 160, 160, 160, 160, 160, 160, 160, 160, 160,
        160, 160, 160, 160, 160, 160, 160, 79, 32, 32, 32, 123, 32, 32, 32, 32, 32, 32, 32, 131,
        160, 97, 229, 160, 160, 160, 160, 160, 160, 160, 160, 160, 160, 160, 160, 160, 160,
    ];

    let bkcolor: [u8; 1000] = [
        14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14,
        14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14,
        14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14,
        14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14,
        14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14,
        14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14,
        14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14,
        14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14,
        14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14,
        14, 14, 43, 43, 43, 43, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14,
        14, 14, 14, 14, 14, 43, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 43, 43, 43, 43,
        14, 15, 15, 15, 15, 15, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14,
        14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 43, 43, 43, 43, 43, 43, 14, 14, 43, 43, 43,
        14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14,
        14, 14, 14, 14, 14, 14, 14, 43, 43, 43, 43, 43, 14, 43, 43, 43, 43, 43, 14, 14, 14, 14, 14,
        14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14,
        14, 15, 15, 15, 15, 43, 14, 43, 43, 43, 43, 43, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14,
        14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 196, 43, 43,
        43, 14, 43, 43, 43, 43, 43, 14, 43, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14,
        14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 196, 196, 14, 43, 43, 43, 43, 43, 43,
        43, 43, 14, 43, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14,
        14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 43, 43, 43, 43, 43, 43, 43, 43, 43, 43, 14, 14,
        14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14,
        14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14,
        14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14,
        14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14,
        14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14,
        14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14,
        14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14,
        14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14,
        14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14,
        14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14,
        14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14,
        14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14,
        14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14,
        14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14,
        14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14,
        14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14,
        14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14,
        14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14,
        14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14,
        14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14,
        14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14,
        14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14, 14,
    ];

    println!("width=40,height=25,texture=upper");
    for i in 0..25 {
        for j in 0..40 {
            print!("{},{} ", bkdata[i * 40 + j], bkcolor[i * 40 + j]);
        }
        println!("");
    }
}