use hex;
use serial_test::serial;

use f1_telemetry::packet::generic::{Flag, WheelData};
use f1_telemetry::packet::motion::{MotionData, PacketMotionData};
use f1_telemetry::packet::session::{
    Formula, MarshalZone, PacketSessionData, SafetyCar, SessionType, Track, Weather,
};
use f1_telemetry::packet::Packet;

mod utils;

#[test]
#[serial]
fn test_parse_2019_motion_packet() {
    let stream = utils::get_stream();
    let socket = utils::get_connected_socket(&stream);

    let data = hex::decode("e30701160100fcc796d4ce430d49000a304269030000138616a1c364d190c245b9d9435bbb124224c4d5bfda2a11c0957f39fa86f7ba088305947ffcc4f73f2ac07b3fb872a13d24ced13f60bd2c3d8d8930bd2d4d96c31ffe81c27abb8d43b02e2bc251edcabf0aca4f4293ae35fdb6624f9dcafd87ae6c82ab3c2618093f0219a8bdaa9930bfc007dfbcd68c8d3c404380c3a3a57ec273c70f43e73185419a90e03e66ad32428c2b41015a78b48758fc912bd20011c0ba0d9d3f535ff73db5d6b13e809ef7bc4842ea3c15c3afc3aea685c28d26b043f8e7edc12157d2bf95973842dfbaecfba46b4a94fe00ddbab0c8243fe34c63c0de0c583ebc1212bf00092fbc8c84febb471a7ec30d667ec2b5974c43a58358c1750297be923a54421dde5cff6d7b978416fe1bde56e128c053f3753f331a533d933689be401581bc2d13753c53d27cc35d2a7ec27d6e1843dbba4b4144d88b3ec65f3b427c1eb1004f7cbb83d6fc7e1e55c820c033b9313f430e04be795c763ec070cfbc5cb9ca3ce2b07ac3f7157ec2ae1f434305091cc163626cbe4e805b4289e67bff6f7d95820dfe87e6929d2cc0fa50733fe29621beef244dbec0ed80bc4a96793c602881c30fd17ec2430c5643508593c12adedabe8a574a42f7d013ff0977fb8820fef5d0dc412cc05c88853f752facbb77b2c0be809385bce740703c293d79c3d3f57dc230fd37439cee81c080c057bda5cc5742ddf2ecff517fb1806afeddf21bf433c0e48c7d3f1d378c3d59a3d2bd00234bbc37274b3ca15480c3b6947ec22fcf5843540097c17794c4be17ca494241d013ffc07644891dfe3fd07bc72ec0deac25be6fe73c3c08c3c3bec05286bc89cb713c8edf81c3c9fb7ec2f9f207437c6da741f30c073f89102842cc3782012e73df8c8afcd137094a14c0fa3d8a3fc2e4113d8410e73e80a1f1bc43a6dd3c157a7bc3da217ec28ab92e438a67533f87152f3c7a564c427efe0500fc7f078053fe7efee9482ec0aff0143f776513bd671641bc00c056bcd9ca563c253391c3c24281c26d1b87437a4f27c22f51c5bf6268504275af37fd6f63949c2afe6aafe65267bf6b8d2c3f52339e3da54d2ebf40c1cbbc74706b3c070bb5c3fee086c27af0b843f32aa0c1d33bb9bf5971234271c886fb3873b48c91006dc89ac5fa3fe9d73bc0a82b69bda6ebe5be804d37bccdb091bb2e5c82c322f27ec2704b60436b17a7c1cbf1d5be79ff4942b2cd11ffb175548ae9fdb0cdca7328c0f8fb5a3f34020c3c25d7cebec08392bc64c8853c98c97ac328157ec2f94a23437f4aeb4052270c3ee06148424b0f5700147ff180d1fd4c0ff86a2bc007886b3f47b7003de658f53dc0978dbc2efd8b3cb40b79c3fff37dc2120d2c43eb2d034034bce83c997a51429c010b00fc7f07802afe9c01f17c25c0f107693f6b8971bd82564e3c80336bbc48256b3cdcf17bc38e397ec243d539431c14a1c0d8955abdc8b4524269f0eaff0a7ff8805afe69f04e2d2ec0f9a5733fe909a63b1524fabd802f53bcfc67533cae6d7dc3ce637ec21de5434329bd1ec1033098be0f4c5242a0e55eff3f7dc582fffd9ee55f3727c0ac31803f10fa91bde59554be801286bcd979803c992e88c3311780c2789a6e4372ef00c2882c5abfec684e428bba45fe7f6b899473fd85badcd4dbbfff0e34bf2df787bc4ce412bf0087c5bc3783a33c07f58741b3b40d414968204146d645404aede6c1726f24c25152534226372042e46cf6c443905cc502674345ca3fc0449ba273427a7372427e0474420cd0724289599eba302ac5ba454e493994dd253922566e3f10ba41bdba5d734290a51a3d0dc037be6277343dc2e5c03f0f9d7c40918c404000000080").unwrap();
    let res = socket.send(&data);

    assert!(res.is_ok());
    assert_eq!(res.unwrap(), data.len());

    let p = stream.next().unwrap().unwrap();

    let actual = match p {
        Packet::Motion(m) => m,
        _ => panic!("Invalid packet. Expected Motion, got {:?}", &p),
    };

    let expected = PacketMotionData::new(
        actual.header().clone(),
        vec![
            MotionData::new(
                -322.17596,
                -72.408966,
                435.44742,
                36.682964,
                -1.6700482,
                -2.2682405,
                32661,
                -1479,
                -2170,
                2234,
                1411,
                32660,
                1.935699,
                0.98340094,
                0.07883209,
                1.6391034,
                0.04217279,
                -0.04309993,
            ),
            MotionData::new(
                -300.60294,
                -64.99633,
                283.46466,
                -42.795593,
                -1.5853673,
                51.947304,
                -20845,
                -715,
                25270,
                -25265,
                -566,
                -20857,
                0.020936213,
                0.5355247,
                -0.08207895,
                -0.6898447,
                -0.027225375,
                0.01727907,
            ),
            MotionData::new(
                -256.5254,
                -63.661755,
                143.7791,
                16.649366,
                0.43860322,
                44.669334,
                11148,
                321,
                30810,
                -30796,
                -936,
                11153,
                -2.265675,
                1.2269814,
                0.120787285,
                0.3473412,
                -0.030226946,
                0.028596058,
            ),
            MotionData::new(
                -351.52408,
                -66.82555,
                352.30118,
                -29.738266,
                -1.643284,
                46.14803,
                -17697,
                -1044,
                27556,
                -27574,
                254,
                -17699,
                0.64368725,
                -3.5515678,
                0.21098658,
                -0.57059836,
                -0.010683298,
                -0.0077672657,
            ),
            MotionData::new(
                -254.10265,
                -63.59966,
                204.5926,
                -13.53214,
                -0.29494062,
                53.057198,
                -8675,
                -164,
                31597,
                -31593,
                -490,
                -8677,
                -2.6387534,
                0.9607441,
                0.051538657,
                -0.2679945,
                -0.015757203,
                0.014958185,
            ),
            MotionData::new(
                -252.82158,
                -63.54137,
                152.4316,
                12.733119,
                0.27313435,
                46.84353,
                7804,
                177,
                31823,
                -31813,
                -810,
                7806,
                -2.5122273,
                0.69423217,
                -0.12896065,
                0.24058713,
                -0.025322318,
                0.02474659,
            ),
            MotionData::new(
                -250.69095,
                -63.52145,
                195.12375,
                -9.752202,
                -0.23084407,
                54.875298,
                -6519,
                -133,
                32111,
                -32107,
                -499,
                -6521,
                -2.6971173,
                0.95045435,
                -0.15780213,
                -0.2003362,
                -0.015738368,
                0.015233586,
            ),
            MotionData::new(
                -258.31543,
                -63.70416,
                214.0479,
                -18.440094,
                -0.42747623,
                50.585487,
                -12041,
                -237,
                30473,
                -30469,
                -480,
                -12043,
                -2.6915197,
                1.0432239,
                -0.0052546808,
                -0.37636158,
                -0.016305685,
                0.014663911,
            ),
            MotionData::new(
                -249.2389,
                -63.490063,
                183.98901,
                -4.060377,
                -0.052673817,
                53.94985,
                -3363,
                -20,
                32593,
                -32591,
                -406,
                -3363,
                -2.811774,
                0.9904311,
                0.068464495,
                -0.10285062,
                -0.012398481,
                0.012399486,
            ),
            MotionData::new(
                -256.66116,
                -63.645226,
                216.80931,
                -18.87516,
                -0.3839452,
                50.447353,
                -12223,
                -237,
                30400,
                -30396,
                -483,
                -12225,
                -2.7309253,
                -0.16179225,
                0.011529787,
                -0.38234735,
                -0.01639688,
                0.014757999,
            ),
            MotionData::new(
                -259.74652,
                -63.745884,
                135.94911,
                20.92846,
                0.52754134,
                42.016148,
                14284,
                386,
                29486,
                -29473,
                -886,
                14289,
                -2.3170187,
                1.0800164,
                0.03561855,
                0.45129788,
                -0.029495955,
                0.027056819,
            ),
            MotionData::new(
                -251.47688,
                -63.53306,
                174.72476,
                0.82579863,
                0.010686285,
                51.08445,
                -386,
                5,
                32764,
                -32761,
                -429,
                -386,
                -2.7232,
                0.58179754,
                -0.035985436,
                -0.011785126,
                -0.0131073,
                0.013109886,
            ),
            MotionData::new(
                -290.39957,
                -64.63039,
                270.21426,
                -41.827614,
                -1.54154,
                52.101936,
                -20619,
                -713,
                25455,
                -25452,
                -470,
                -20630,
                -0.9036087,
                0.67403287,
                0.07724632,
                -0.68087226,
                -0.024872422,
                0.014370073,
            ),
            MotionData::new(
                -362.08615,
                -67.43944,
                369.87872,
                -20.020971,
                -1.4471382,
                40.86069,
                -14223,
                -1146,
                29496,
                -29516,
                145,
                -14227,
                1.9591553,
                -2.935053,
                -0.0569264,
                -0.44906348,
                -0.011187911,
                -0.004446125,
            ),
            MotionData::new(
                -260.72015,
                -63.736458,
                224.29468,
                -20.886435,
                -0.41786036,
                50.499485,
                -12878,
                -239,
                30129,
                -30124,
                -535,
                -12880,
                -2.6320672,
                0.85540724,
                0.008545447,
                -0.40398517,
                -0.017885089,
                0.016330905,
            ),
            MotionData::new(
                -250.78748,
                -63.52066,
                163.29286,
                7.3528438,
                0.13686875,
                50.09558,
                3915,
                87,
                32532,
                -32527,
                -559,
                3916,
                -2.6784039,
                0.92004436,
                0.031424787,
                0.11979847,
                -0.017284274,
                0.017088499,
            ),
            MotionData::new(
                -249.04572,
                -63.488277,
                172.05106,
                2.0496776,
                0.028410055,
                52.369724,
                412,
                11,
                32764,
                -32761,
                -470,
                412,
                -2.5857508,
                0.9102774,
                -0.058968943,
                0.012593867,
                -0.01435554,
                0.01435215,
            ),
            MotionData::new(
                -251.94476,
                -63.556206,
                185.83305,
                -5.0337048,
                -0.05336556,
                52.676544,
                -3991,
                -22,
                32522,
                -32520,
                -422,
                -3991,
                -2.7215152,
                0.9517513,
                0.0050670994,
                -0.12213913,
                -0.012889743,
                0.01290321,
            ),
            MotionData::new(
                -253.42844,
                -63.597466,
                195.89497,
                -9.921182,
                -0.2972413,
                52.574276,
                -6752,
                -162,
                32063,
                -32059,
                -513,
                -6754,
                -2.6127546,
                1.0015159,
                -0.07127774,
                -0.20760305,
                -0.016366243,
                0.015683101,
            ),
            MotionData::new(
                -272.36404,
                -64.045296,
                238.6034,
                -32.233833,
                -0.852242,
                51.602463,
                -17781,
                -443,
                27519,
                -27511,
                -653,
                -17787,
                -1.7174335,
                -0.7033538,
                -0.016597355,
                -0.57379603,
                -0.024112225,
                0.019960029,
            ),
        ],
        WheelData::new(16.994642, 8.856616, 10.02546, 3.0912032),
        WheelData::new(-28.865864, -41.108833, 52.830387, 40.053856),
        WheelData::new(-1971.4028, -3529.0164, 3126.438, 1537.9934),
        WheelData::new(60.908794, 60.61277, 61.004387, 60.70317),
        WheelData::new(-0.0012081127, -0.001504248, 0.00019198011, 0.0001581817),
        0.9310018,
        -0.047296584,
        60.84153,
        0.03775555,
        -0.17944355,
        0.044059165,
        1.5070117,
        3.947086,
        3.0085795,
        -0.0,
    );

    assert_eq!(expected, actual);
}

#[test]
#[serial]
fn test_parse_2019_session_packet() {
    let stream = utils::get_stream();
    let socket = utils::get_connected_socket(&stream);

    let data = hex::decode("e30701160101fcc796d4ce430d49000a3042690300001301211a052a120a0400fb1b201c500000ff00126a46783f0076db943d019c7f1b3e02d0f75d3e039aa3763e04fb8aa83e00dc2ec13e0050dcd83e00bddf043f00074e0f3f00591b1b3f008fb6243f0010a6383f00e3f0473f000bea4d3f00ef595a3f001301623f00f39e683f000000000000000000000000000000000000").unwrap();
    let res = socket.send(&data);

    assert!(res.is_ok());
    assert_eq!(res.unwrap(), data.len());

    let p = stream.next().unwrap().unwrap();

    let actual = match p {
        Packet::Session(s) => s,
        _ => panic!("Invalid packet. Expected Session, got {:?}", &p),
    };

    let expected = PacketSessionData::new(
        actual.header().clone(),
        Weather::LightCloud,
        33,
        26,
        5,
        4650,
        SessionType::Race,
        Track::Catalunya,
        Formula::F1Modern,
        7163,
        7200,
        80,
        false,
        false,
        255,
        false,
        18,
        vec![
            MarshalZone::new(0.96982443, Flag::None),
            MarshalZone::new(0.07268421, Flag::Green),
            MarshalZone::new(0.15185398, Flag::Blue),
            MarshalZone::new(0.21676564, Flag::Yellow),
            MarshalZone::new(0.24085847, Flag::Red),
            MarshalZone::new(0.32918534, Flag::None),
            MarshalZone::new(0.37731063, Flag::None),
            MarshalZone::new(0.42355585, Flag::None),
            MarshalZone::new(0.519039, Flag::None),
            MarshalZone::new(0.55978435, Flag::None),
            MarshalZone::new(0.60588604, Flag::None),
            MarshalZone::new(0.6434106, Flag::None),
            MarshalZone::new(0.7212839, Flag::None),
            MarshalZone::new(0.7810194, Flag::None),
            MarshalZone::new(0.80435246, Flag::None),
            MarshalZone::new(0.8529348, Flag::None),
            MarshalZone::new(0.8828289, Flag::None),
            MarshalZone::new(0.9086754, Flag::None),
            MarshalZone::new(0.0, Flag::None),
            MarshalZone::new(0.0, Flag::None),
            MarshalZone::new(0.0, Flag::None),
        ],
        SafetyCar::None,
        false,
        None,
        None,
    );

    assert_eq!(actual, expected);
}

#[test]
#[serial]
fn test_parse_2019_lap_packet() {
    let stream = utils::get_stream();
    let socket = utils::get_connected_socket(&stream);

    let data = hex::decode("e30701160102fcc796d4ce430d49000a30426903000013000000007e5e11420000000034a6c84100000000db460845db4608450000008001010001000000040200000000625f1142000000006605ee41000000003f41f6443f41f6440000008004010001000003040200000000f15e114200000000764cfc4100000000724ee344724ee3440000008013010001000012040200000000ff671142000000001018e241000000008a7800458a7800450000008003010001000006040200000000dc5f1142000000007e07f4410000000084dcea4484dcea44000000800a010001000008040200000000f36511420000000082e1fa41000000003c72e4443c72e4440000008012010001000011040200000000c968114200000000283bf54100000000a4a7e944a4a7e944000000800c01000100000a040200000000195a11420000000085eaf04100000000831fec44831fec4400000080090100010000010402000000002e571142000000002aacf64100000000484de844484de844000000800e01000100000c04020000000099601142000000009d2af04100000000385dec44385dec440000008008010001000007040200000000e062114200000000e2effd41000000008f3fe2448f3fe24400000080140100010000130402000000004060114200000000e01af841000000000530e7440530e744000000800f01000100000e040200000000bf68114200000000cd08ef4100000000df2bf444df2bf44400000080050100010000050402000000008a601142000000008cf6e0410000000012c0014512c0014500000080020100010000040402000000000d67114200000000014af141000000000e63ed440e63ed44000000800701000100000b0402000000003568114200000000586af941000000008bcbe5448bcbe544000000801101000100000f040200000000ab60114200000000ef13f9410000000050dde64450dde6440000008010010001000010040200000000955f1142000000001076f64100000000178fe844178fe844000000800d01000100000d0402000000001666114200000000c135f5410000000044cfe94444cfe944000000800b0100010000090402000000006469114200000000bd80f241000000007da1ef447da1ef4400000080060100010000020402").unwrap();
    let res = socket.send(&data);

    assert!(res.is_ok());
    assert_eq!(res.unwrap(), data.len());

    let p = stream.next().unwrap().unwrap();

    assert!(matches!(p, Packet::Lap(_)));
}

#[test]
#[serial]
fn test_parse_2019_event_packet() {
    let stream = utils::get_stream();
    let socket = utils::get_connected_socket(&stream);

    let data =
        hex::decode("e30701160103fcc796d4ce430d49801cb542f20600001346544c50007bd4a542").unwrap();
    let res = socket.send(&data);

    assert!(res.is_ok());
    assert_eq!(res.unwrap(), data.len());

    let p = stream.next().unwrap().unwrap();

    assert!(matches!(p, Packet::Event(_)));
}

#[test]
#[serial]
fn test_parse_2019_participants_packet() {
    let stream = utils::get_stream();
    let socket = utils::get_connected_socket(&stream);

    let data = hex::decode("e30701160104fcc796d4ce430d49000a3042690300001314010f004d1b562e20424f545441530000000000000000000000000000000000000000000000000000000000000000000000000000000101090221164d2e205645525354415050454e000000000000000000000000000000000000000000000000000000000000000000000001010609071b4b2e2052c384494b4bc3964e454e0000000000000000000000000000000000000000000000000000000000000000000001010a051b1d4e2e2048c39c4c4b454e4245524700000000000000000000000000000000000000000000000000000000000000000000010102050303442e2052494343494152444f00000000000000000000000000000000000000000000000000000000000000000000000001014b035840522e204b554249434100000000000000000000000000000000000000000000000000000000000000000000000000000001010008374e432e205341494e5a0000000000000000000000000000000000000000000000000000000000000000000000000000000001013a011035432e204c45434c455243000000000000000000000000000000000000000000000000000000000000000000000000000001010b0714154b2e204d41474e555353454e00000000000000000000000000000000000000000000000000000000000000000000000001010e040b34532e20504552455a00000000000000000000000000000000000000000000000000000000000000000000000000000000010101061a44442e204b5659415400000000000000000000000000000000000000000000000000000000000000000000000000000000010132033f0a472e2052555353454c4c000000000000000000000000000000000000000000000000000000000000000000000000000001013e021751412e20414c424f4e0000000000000000000000000000000000000000000000000000000000000000000000000000000001010d01051d532e2056455454454c00000000000000000000000000000000000000000000000000000000000000000000000000000001011304120d4c2e205354524f4c4c00000000000000000000000000000000000000000000000000000000000000000000000000000001014a096329412e2047494f56494e415a5a49000000000000000000000000000000000000000000000000000000000000000000000001013b060a1c502e204741534c590000000000000000000000000000000000000000000000000000000000000000000000000000000001010c07081c522e2047524f534a45414e0000000000000000000000000000000000000000000000000000000000000000000000000001013608040a4c2e204e4f52524953000000000000000000000000000000000000000000000000000000000000000000000000000000010007002c0a4c2e2048414d494c544f4e0000000000000000000000000000000000000000000000000000000000000000000000000000").unwrap();
    let res = socket.send(&data);

    assert!(res.is_ok());
    assert_eq!(res.unwrap(), data.len());

    let p = stream.next().unwrap().unwrap();

    assert!(matches!(p, Packet::Participants(_)));
}

#[test]
#[serial]
fn test_parse_2019_car_setups_packet() {
    let stream = utils::get_stream();
    let socket = utils::get_connected_socket(&stream);

    let data = hex::decode("e30701160105fcc796d4ce430d49000a3042690300001306064b4b000040c00000c0bfcecccc3d3433b33e0606060606064b3c0000b8410000ac41060000204106064b4b000040c00000c0bfcecccc3d3433b33e0606060606064b3c0000b8410000ac41060000204106064b4b000040c00000c0bfcecccc3d3433b33e0606060606064b3c0000b8410000ac41060000204106064b4b000040c00000c0bfcecccc3d3433b33e0606060606064b3c0000b8410000ac41060000204106064b4b000040c00000c0bfcecccc3d3433b33e0606060606064b3c0000b8410000ac41060000204106064b4b000040c00000c0bfcecccc3d3433b33e0606060606064b3c0000b8410000ac41060000204106064b4b000040c00000c0bfcecccc3d3433b33e0606060606064b3c0000b8410000ac41060000204106064b4b000040c00000c0bfcecccc3d3433b33e0606060606064b3c0000b8410000ac41060000204106064b4b000040c00000c0bfcecccc3d3433b33e0606060606064b3c0000b8410000ac41060000204106064b4b000040c00000c0bfcecccc3d3433b33e0606060606064b3c0000b8410000ac41060000204106064b4b000040c00000c0bfcecccc3d3433b33e0606060606064b3c0000b8410000ac41060000204106064b4b000040c00000c0bfcecccc3d3433b33e0606060606064b3c0000b8410000ac41060000204106064b4b000040c00000c0bfcecccc3d3433b33e0606060606064b3c0000b8410000ac41060000204106064b4b000040c00000c0bfcecccc3d3433b33e0606060606064b3c0000b8410000ac41060000204106064b4b000040c00000c0bfcecccc3d3433b33e0606060606064b3c0000b8410000ac41060000204106064b4b000040c00000c0bfcecccc3d3433b33e0606060606064b3c0000b8410000ac41060000204106064b4b000040c00000c0bfcecccc3d3433b33e0606060606064b3c0000b8410000ac41060000204106064b4b000040c00000c0bfcecccc3d3433b33e0606060606064b3c0000b8410000ac41060000204106064b4b000040c00000c0bfcecccc3d3433b33e0606060606064b3c0000b8410000ac41060000204106064b4b000040c00000c0bfcecccc3d3433b33e0606060606064b3c0000b8410000ac410600002041").unwrap();
    let res = socket.send(&data);

    assert!(res.is_ok());
    assert_eq!(res.unwrap(), data.len());

    let p = stream.next().unwrap().unwrap();

    assert!(matches!(p, Packet::CarSetups(_)));
}

#[test]
#[serial]
fn test_parse_2019_car_telemetry_packet() {
    let stream = utils::get_stream();
    let socket = utils::get_connected_socket(&stream);

    let data = hex::decode("e30701160106fcc796d4ce430d49000a304269030000138400c2b0163f354129be000000000003b6280000470349038e0391035f005f0061006a0062005d0060005b005a000000ac410000ac410000b8410000b84100000000f2000000803fac9802bc000000000006f82a000850024d026d02690269005c006b0056005f005b005d0058005a000000ac410000ac410000b8410000b84100000000ab000000803f9398413e000000000004862a0000b702b602ed02ec02610056006c0053005b005900590055005a000000ac410000ac410000b8410000b84100000000c50000000000db9bfebc0000803f0005b1280000160316037103720362005900660055005f005b005e0057005a000000ac410000ac410000b8410000b84100000000c5000000803fd4041c3e000000000005a7290000d702d3020b03080364005900670052005d0059005b0057005a000000ac410000ac410000b8410000b84100000000ae00a7cc493fd9d66a3e0000000000044a2b000bb302b102e802e70261005600670050005b0058005a0055005a000000ac410000ac410000b8410000b84100000000c8000000803f25bf2a3e000000000005182b000bd102ce020303010367005a006a0053005b0059005b0057005a000000ac410000ac410000b8410000b84100000000c100e5667c3f341dd03d00000000000535290000d202cf020203000365005800720058005d0059005e0058005a000000ac410000ac410000b8410000b84100000000c20076f77d3fbb77183e00000000000567290000b802b502e902e702660059006a0052005c0059005b0057005a000000ac410000ac410000b8410000b84100000000c1000000803ff594ea3d000000000004262e005dbc02b902f302f002640058006d0054005d005a005d0057005a000000ac410000ac410000b8410000b84100000000a80083f6713f0f61583e000000000004d0290000be02bd02f502f50260005500670052005b0059005a0056005a000000ac410000ac410000b8410000b84100000000b7003006433fc75c4a3e0000000000043b2d0042ae02ad02e002de0265005800690051005d0059005b0057005a000000ac410000ac410000b8410000b84100000000f0000000803f6fc49c3c000000000006ac2a0000590256027802750269005c006c0055005e005a005c0057005a000000ac410000ac410000b8410000b84100000000a30000000000ca2f4dbe0000803f0004692600005e036003be03c10362005b0068005a0060005c005e0058005a000000ac410000ac410000b8410000b84100000000c4004a670e3f26cb8f3e0000000000051a280000ba02b702e902e60267005900700055005e005a005c0057005a000000ac410000ac410000b8410000b84100000000b600e30e7f3f2de63a3e000000000004612d002fb002af02e402e30263005700680051005c0059005a0057005a000000ac410000ac410000b8410000b84100000000bc0077c6a23eb2d3363e0000000000052b2800009e029c02d002cf02650057006a0052005c0059005a0056005a000000ac410000ac410000b8410000b84100000000be003e9b6a3f70a1183e00000000000574280000cb02c802ff02fc0265005900690052005c0059005b0057005a000000ac410000ac410000b8410000b84100000000c000a0527f3f0e9b263e00000000000557290000d702d3020d030a0364005900680053005c0059005b0057005a000000ac410000ac410000b8410000b84100000000db000000000000000000000000000005002c001eb802b602d502d302700061007200560061005d005f0059006a000000ac410000ac410000b8410000b8410000000000000000").unwrap();
    let res = socket.send(&data);

    assert!(res.is_ok());
    assert_eq!(res.unwrap(), data.len());

    let p = stream.next().unwrap().unwrap();

    assert!(matches!(p, Packet::CarTelemetry(_)));
}

#[test]
#[serial]
fn test_parse_2019_car_status_packet() {
    let stream = utils::get_stream();
    let socket = utils::get_connected_socket(&stream);

    let data = hex::decode("e30701160107fcc796d4ce430d49000a304269030000130001023c001ecc14410000dc4234729f3f2035cc100900020101011210020101010000000000009df03d4a035bd9a948e4f7c3485cc5c7490001003c00d67813410000dc42c8fb923fbc34d70e0900010101001210010101000000000000003f00424a03b504a748d7c9dc484f23c5490001013c001a6217410000dc420078a23fbc34cc100900010100001210010100000000000000009457544a041668a4487dd117490a83b4490001003c00b4ee15410000dc42b0b3a13fbc34d70e0900010101001210010101000000000000007924364a0428dcaf48a6b3d548ce4add490001003c0054f115410000dc4290e49c3fbc34d70e0900010101001210010101000000000000000b504e4a0476cca8482338084910deb9490001003c0084910e410000dc4260e06c3f2035cc100900010100001210010100000000000000009b74514a0422299a481bab14496a26b6490001013c001e0219410000dc42f406ac3fbc34ab0d0900010101001210010101000000000000007a284d4a041ebea94880170b491fdabd490001013c00bb6717410000dc429084a43fbc34cc100900010101001210010101000000000000007c68564a0434e5ac489db411494b73af490001003c00c1260e410000dc4218616a3fbc34cc10090001010100121001010100000000000000d8234b4a047ce8a248142b0749ca37be490001003c00b9d015410000dc42a4959c3f2035cc10090001010100121001010100000000000000ff6e514a04b7b5a648d2140b49138ab4490001003c0005a013410000dc42485b8f3fbc34ab0d090001010000121001010000000000000000111f4f4a044c6d9f486f3a1149bc67ba490001003c0008e414410000dc4208cd963f2035cc100900010101001210010101000000000000002f914f4a04326ca34868870e49a02ab9490001013c00ae4f18410000dc42c8d5aa3fbc34d70e090001010100121001010100000000000000f25a424a04811b9f484269f4488f59c8490001003c0082db12410000dc4294d2923fbc34cc100900010101001210010101000000000000000ef83f4a039f46b74852ced8483a42cc490001013c004ce117410000dc42782ea73f2035cc10090001010100121001010100000000000000ddba4f4a044c28a848997709499e7eb7490001003c00ff6013410000dc4254e68e3fbc34cc10090001010000121001010000000000000000eb9e4f4a044164974865bb0d4959a9b5490001003c0087df0c410000dc42e0e75c3fbc34ab0d090001010000121001010000000000000000bacb4a4a04d49f9c48150b09497243be490001013c00010018410000dc42acb8a63fbc34cc100900010101001210010101000000000000008788524a04fdf6a6483a991449e027b7490001003c009eb215410000dc42586e9b3fbc34ab0d090001010100121001010100000000000000df0a4d4a045e49a448427d074918e8ba490101013c00225114410000dc42b4c5953f2035cc10090001010100121001010100180000000000d3692e4a057e0bb248c323cb4860c0ea49").unwrap();
    let res = socket.send(&data);

    assert!(res.is_ok());
    assert_eq!(res.unwrap(), data.len());

    let p = stream.next().unwrap().unwrap();

    assert!(matches!(p, Packet::CarStatus(_)));
}
