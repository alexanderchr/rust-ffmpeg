#[crate_id = "avformat#54"];
#[license = "MIT"];
#[crate_type = "dylib"];
/* automatically generated by rust-bindgen */
extern crate avutil;
extern crate avcodec;
use std::libc::{c_void,c_int,c_uint,c_schar,uint8_t,int64_t,uint64_t,c_uchar,c_ulong,FILE};

pub type __int128_t = c_void;
pub type __uint128_t = c_void;
pub type __builtin_va_list = [__va_list_tag, ..1u];
pub struct Struct_AVIOInterruptCB {
    callback: extern "C" fn(arg1: *mut c_void) -> c_int,
    opaque: *mut c_void,
}
pub type AVIOInterruptCB = Struct_AVIOInterruptCB;
pub struct Struct_AVIOContext {
    av_class: *avutil::AVClass,
    buffer: *mut c_uchar,
    buffer_size: c_int,
    buf_ptr: *mut c_uchar,
    buf_end: *mut c_uchar,
    opaque: *mut c_void,
    read_packet: extern "C" fn
                     (arg1: *mut c_void, arg2: *mut uint8_t, arg3: c_int)
                     -> c_int,
    write_packet: extern "C" fn
                      (arg1: *mut c_void, arg2: *mut uint8_t, arg3: c_int)
                      -> c_int,
    seek: extern "C" fn(arg1: *mut c_void, arg2: int64_t, arg3: c_int)
              -> int64_t,
    pos: int64_t,
    must_flush: c_int,
    eof_reached: c_int,
    write_flag: c_int,
    max_packet_size: c_int,
    checksum: c_ulong,
    checksum_ptr: *mut c_uchar,
    update_checksum: extern "C" fn
                         (arg1: c_ulong, arg2: *uint8_t, arg3: c_uint)
                         -> c_ulong,
    error: c_int,
    read_pause: extern "C" fn(arg1: *mut c_void, arg2: c_int) -> c_int,
    read_seek: extern "C" fn
                   (arg1: *mut c_void, arg2: c_int, arg3: int64_t,
                    arg4: c_int) -> int64_t,
    seekable: c_int,
    maxsize: int64_t,
    direct: c_int,
    bytes_read: int64_t,
    seek_count: c_int,
}
pub type AVIOContext = Struct_AVIOContext;
pub struct Struct_AVFrac {
    val: int64_t,
    num: int64_t,
    den: int64_t,
}
pub type AVFrac = Struct_AVFrac;
pub type Struct_AVCodecTag = c_void;
pub struct Struct_AVProbeData {
    filename: *c_schar,
    buf: *mut c_uchar,
    buf_size: c_int,
}
pub type AVProbeData = Struct_AVProbeData;
pub struct Struct_AVOutputFormat {
    name: *c_schar,
    long_name: *c_schar,
    mime_type: *c_schar,
    extensions: *c_schar,
    audio_codec: avcodec::Enum_AVCodecID,
    video_codec: avcodec::Enum_AVCodecID,
    subtitle_codec: avcodec::Enum_AVCodecID,
    flags: c_int,
    codec_tag: **Struct_AVCodecTag,
    priv_class: *avutil::AVClass,
    next: *mut Struct_AVOutputFormat,
    priv_data_size: c_int,
    write_header: extern "C" fn(arg1: *mut Struct_AVFormatContext) -> c_int,
    write_packet: extern "C" fn
                      (arg1: *mut Struct_AVFormatContext, arg2: *mut avcodec::AVPacket)
                      -> c_int,
    write_trailer: extern "C" fn(arg1: *mut Struct_AVFormatContext) -> c_int,
    interleave_packet: extern "C" fn
                           (arg1: *mut Struct_AVFormatContext,
                            arg2: *mut avcodec::AVPacket, arg3: *mut avcodec::AVPacket,
                            arg4: c_int) -> c_int,
    query_codec: extern "C" fn(arg1: avcodec::Enum_AVCodecID, arg2: c_int) -> c_int,
    get_output_timestamp: extern "C" fn
                              (arg1: *mut Struct_AVFormatContext, arg2: c_int,
                               arg3: *mut int64_t, arg4: *mut int64_t),
}
pub type AVOutputFormat = Struct_AVOutputFormat;
pub struct Struct_AVInputFormat {
    name: *c_schar,
    long_name: *c_schar,
    flags: c_int,
    extensions: *c_schar,
    codec_tag: **Struct_AVCodecTag,
    priv_class: *avutil::AVClass,
    next: *mut Struct_AVInputFormat,
    raw_codec_id: c_int,
    priv_data_size: c_int,
    read_probe: extern "C" fn(arg1: *mut AVProbeData) -> c_int,
    read_header: extern "C" fn(arg1: *mut Struct_AVFormatContext) -> c_int,
    read_packet: extern "C" fn
                     (arg1: *mut Struct_AVFormatContext, arg2: *mut avcodec::AVPacket)
                     -> c_int,
    read_close: extern "C" fn(arg1: *mut Struct_AVFormatContext) -> c_int,
    read_seek: extern "C" fn
                   (arg1: *mut Struct_AVFormatContext, arg2: c_int,
                    arg3: int64_t, arg4: c_int) -> c_int,
    read_timestamp: extern "C" fn
                        (arg1: *mut Struct_AVFormatContext, arg2: c_int,
                         arg3: *mut int64_t, arg4: int64_t) -> int64_t,
    read_play: extern "C" fn(arg1: *mut Struct_AVFormatContext) -> c_int,
    read_pause: extern "C" fn(arg1: *mut Struct_AVFormatContext) -> c_int,
    read_seek2: extern "C" fn
                    (arg1: *mut Struct_AVFormatContext, arg2: c_int,
                     arg3: int64_t, arg4: int64_t, arg5: int64_t, arg6: c_int)
                    -> c_int,
}
pub type AVInputFormat = Struct_AVInputFormat;
pub type Enum_AVStreamParseType = c_uint;
pub static AVSTREAM_PARSE_NONE: c_uint = 0;
pub static AVSTREAM_PARSE_FULL: c_uint = 1;
pub static AVSTREAM_PARSE_HEADERS: c_uint = 2;
pub static AVSTREAM_PARSE_TIMESTAMPS: c_uint = 3;
pub static AVSTREAM_PARSE_FULL_ONCE: c_uint = 4;
pub static AVSTREAM_PARSE_FULL_RAW: c_uint = 1463898624;
pub struct Struct_AVIndexEntry {
    pos: int64_t,
    timestamp: int64_t,
    flags: c_int,
    size: c_int,
    min_distance: c_int,
}
pub type AVIndexEntry = Struct_AVIndexEntry;
pub struct Struct_AVStream {
    index: c_int,
    id: c_int,
    codec: *mut avcodec::AVCodecContext,
    r_frame_rate: avutil::AVRational,
    priv_data: *mut c_void,
    pts: Struct_AVFrac,
    time_base: avutil::AVRational,
    start_time: int64_t,
    duration: int64_t,
    nb_frames: int64_t,
    disposition: c_int,
    discard: avcodec::Enum_AVDiscard,
    sample_aspect_ratio: avutil::AVRational,
    metadata: *mut avutil::AVDictionary,
    avg_frame_rate: avutil::AVRational,
    attached_pic: avcodec::AVPacket,
    info: *mut Struct_Unnamed1,
    pts_wrap_bits: c_int,
    reference_dts: int64_t,
    first_dts: int64_t,
    cur_dts: int64_t,
    last_IP_pts: int64_t,
    last_IP_duration: c_int,
    probe_packets: c_int,
    codec_info_nb_frames: c_int,
    stream_identifier: c_int,
    interleaver_chunk_size: int64_t,
    interleaver_chunk_duration: int64_t,
    need_parsing: Enum_AVStreamParseType,
    parser: *mut avcodec::Struct_AVCodecParserContext,
    last_in_packet_buffer: *mut Struct_AVPacketList,
    probe_data: AVProbeData,
    pts_buffer: [int64_t, ..17u],
    index_entries: *mut AVIndexEntry,
    nb_index_entries: c_int,
    index_entries_allocated_size: c_uint,
    request_probe: c_int,
    skip_to_keyframe: c_int,
    skip_samples: c_int,
    nb_decoded_frames: c_int,
    mux_ts_offset: int64_t,
    pts_wrap_reference: int64_t,
    pts_wrap_behavior: c_int,
}
pub struct Struct_Unnamed1 {
    last_dts: int64_t,
    duration_gcd: int64_t,
    duration_count: c_int,
    duration_error: *mut c_void,
    codec_info_duration: int64_t,
    codec_info_duration_fields: int64_t,
    found_decoder: c_int,
    last_duration: int64_t,
    fps_first_dts: int64_t,
    fps_first_dts_idx: c_int,
    fps_last_dts: int64_t,
    fps_last_dts_idx: c_int,
}
pub type AVStream = Struct_AVStream;
pub struct Struct_AVProgram {
    id: c_int,
    flags: c_int,
    discard: avcodec::Enum_AVDiscard,
    stream_index: *mut c_uint,
    nb_stream_indexes: c_uint,
    metadata: *mut avutil::AVDictionary,
    program_num: c_int,
    pmt_pid: c_int,
    pcr_pid: c_int,
    start_time: int64_t,
    end_time: int64_t,
    pts_wrap_reference: int64_t,
    pts_wrap_behavior: c_int,
}
pub type AVProgram = Struct_AVProgram;
pub struct Struct_AVChapter {
    id: c_int,
    time_base: avutil::AVRational,
    start: int64_t,
    end: int64_t,
    metadata: *mut avutil::AVDictionary,
}
pub type AVChapter = Struct_AVChapter;
pub type Enum_AVDurationEstimationMethod = c_uint;
pub static AVFMT_DURATION_FROM_PTS: c_uint = 0;
pub static AVFMT_DURATION_FROM_STREAM: c_uint = 1;
pub static AVFMT_DURATION_FROM_BITRATE: c_uint = 2;
pub struct Struct_AVFormatContext {
    av_class: *avutil::AVClass,
    iformat: *mut Struct_AVInputFormat,
    oformat: *mut Struct_AVOutputFormat,
    priv_data: *mut c_void,
    pb: *mut AVIOContext,
    ctx_flags: c_int,
    nb_streams: c_uint,
    streams: *mut *mut AVStream,
    filename: [c_schar, ..1024u],
    start_time: int64_t,
    duration: int64_t,
    bit_rate: c_int,
    packet_size: c_uint,
    max_delay: c_int,
    flags: c_int,
    probesize: c_uint,
    max_analyze_duration: c_int,
    key: *uint8_t,
    keylen: c_int,
    nb_programs: c_uint,
    programs: *mut *mut AVProgram,
    video_codec_id: avcodec::Enum_AVCodecID,
    audio_codec_id: avcodec::Enum_AVCodecID,
    subtitle_codec_id: avcodec::Enum_AVCodecID,
    max_index_size: c_uint,
    max_picture_buffer: c_uint,
    nb_chapters: c_uint,
    chapters: *mut *mut AVChapter,
    metadata: *mut avutil::AVDictionary,
    start_time_realtime: int64_t,
    fps_probe_size: c_int,
    error_recognition: c_int,
    interrupt_callback: AVIOInterruptCB,
    debug: c_int,
    ts_id: c_int,
    audio_preload: c_int,
    max_chunk_duration: c_int,
    max_chunk_size: c_int,
    use_wallclock_as_timestamps: c_int,
    avoid_negative_ts: c_int,
    avio_flags: c_int,
    duration_estimation_method: Enum_AVDurationEstimationMethod,
    skip_initial_bytes: c_uint,
    correct_ts_overflow: c_uint,
    seek2any: c_int,
    packet_buffer: *mut Struct_AVPacketList,
    packet_buffer_end: *mut Struct_AVPacketList,
    data_offset: int64_t,
    raw_packet_buffer: *mut Struct_AVPacketList,
    raw_packet_buffer_end: *mut Struct_AVPacketList,
    parse_queue: *mut Struct_AVPacketList,
    parse_queue_end: *mut Struct_AVPacketList,
    raw_packet_buffer_remaining_size: c_int,
}
pub type AVFormatContext = Struct_AVFormatContext;
pub struct Struct_AVPacketList {
    pkt: avcodec::AVPacket,
    next: *mut Struct_AVPacketList,
}
pub type AVPacketList = Struct_AVPacketList;
pub type __va_list_tag = Struct___va_list_tag;
pub struct Struct___va_list_tag {
    gp_offset: c_uint,
    fp_offset: c_uint,
    overflow_arg_area: *mut c_void,
    reg_save_area: *mut c_void,
}
pub static AVSEEK_FLAG_BACKWARD: c_int = 1;
pub static AVSEEK_FLAG_BYTE: c_int = 2;
pub static AVSEEK_FLAG_ANY: c_int = 4;
pub static AVSEEK_FLAG_FRAME: c_int = 8;
#[link(name = "avformat")]
extern "C" {
    pub fn avio_check(url: *c_schar, flags: c_int) -> c_int;
    pub fn avio_alloc_context(buffer: *mut c_uchar, buffer_size: c_int,
                              write_flag: c_int, opaque: *mut c_void,
                              read_packet:
                                  extern "C" fn
                                      (arg1: *mut c_void, arg2: *mut uint8_t,
                                       arg3: c_int) -> c_int,
                              write_packet:
                                  extern "C" fn
                                      (arg1: *mut c_void, arg2: *mut uint8_t,
                                       arg3: c_int) -> c_int,
                              seek:
                                  extern "C" fn
                                      (arg1: *mut c_void, arg2: int64_t,
                                       arg3: c_int) -> int64_t) ->
     *mut AVIOContext;
    pub fn avio_w8(s: *mut AVIOContext, b: c_int);
    pub fn avio_write(s: *mut AVIOContext, buf: *c_uchar, size: c_int);
    pub fn avio_wl64(s: *mut AVIOContext, val: uint64_t);
    pub fn avio_wb64(s: *mut AVIOContext, val: uint64_t);
    pub fn avio_wl32(s: *mut AVIOContext, val: c_uint);
    pub fn avio_wb32(s: *mut AVIOContext, val: c_uint);
    pub fn avio_wl24(s: *mut AVIOContext, val: c_uint);
    pub fn avio_wb24(s: *mut AVIOContext, val: c_uint);
    pub fn avio_wl16(s: *mut AVIOContext, val: c_uint);
    pub fn avio_wb16(s: *mut AVIOContext, val: c_uint);
    pub fn avio_put_str(s: *mut AVIOContext, str: *c_schar) -> c_int;
    pub fn avio_put_str16le(s: *mut AVIOContext, str: *c_schar) -> c_int;
    pub fn avio_seek(s: *mut AVIOContext, offset: int64_t, whence: c_int) ->
     int64_t;
    pub fn avio_skip(s: *mut AVIOContext, offset: int64_t) -> int64_t;
    pub fn avio_size(s: *mut AVIOContext) -> int64_t;
    pub fn url_feof(s: *mut AVIOContext) -> c_int;
    pub fn avio_printf(s: *mut AVIOContext, fmt: *c_schar) -> c_int;
    pub fn avio_flush(s: *mut AVIOContext);
    pub fn avio_read(s: *mut AVIOContext, buf: *mut c_uchar, size: c_int) ->
     c_int;
    pub fn avio_r8(s: *mut AVIOContext) -> c_int;
    pub fn avio_rl16(s: *mut AVIOContext) -> c_uint;
    pub fn avio_rl24(s: *mut AVIOContext) -> c_uint;
    pub fn avio_rl32(s: *mut AVIOContext) -> c_uint;
    pub fn avio_rl64(s: *mut AVIOContext) -> uint64_t;
    pub fn avio_rb16(s: *mut AVIOContext) -> c_uint;
    pub fn avio_rb24(s: *mut AVIOContext) -> c_uint;
    pub fn avio_rb32(s: *mut AVIOContext) -> c_uint;
    pub fn avio_rb64(s: *mut AVIOContext) -> uint64_t;
    pub fn avio_get_str(pb: *mut AVIOContext, maxlen: c_int,
                        buf: *mut c_schar, buflen: c_int) -> c_int;
    pub fn avio_get_str16le(pb: *mut AVIOContext, maxlen: c_int,
                            buf: *mut c_schar, buflen: c_int) -> c_int;
    pub fn avio_get_str16be(pb: *mut AVIOContext, maxlen: c_int,
                            buf: *mut c_schar, buflen: c_int) -> c_int;
    pub fn avio_open(s: *mut *mut AVIOContext, url: *c_schar, flags: c_int) ->
     c_int;
    pub fn avio_open2(s: *mut *mut AVIOContext, url: *c_schar, flags: c_int,
                      int_cb: *AVIOInterruptCB,
                      options: *mut *mut avutil::AVDictionary) -> c_int;
    pub fn avio_close(s: *mut AVIOContext) -> c_int;
    pub fn avio_closep(s: *mut *mut AVIOContext) -> c_int;
    pub fn avio_open_dyn_buf(s: *mut *mut AVIOContext) -> c_int;
    pub fn avio_close_dyn_buf(s: *mut AVIOContext, pbuffer: *mut *mut uint8_t)
     -> c_int;
    pub fn avio_enum_protocols(opaque: *mut *mut c_void, output: c_int) ->
     *c_schar;
    pub fn avio_pause(h: *mut AVIOContext, pause: c_int) -> c_int;
    pub fn avio_seek_time(h: *mut AVIOContext, stream_index: c_int,
                          timestamp: int64_t, flags: c_int) -> int64_t;
    pub fn av_get_packet(s: *mut AVIOContext, pkt: *mut avcodec::AVPacket, size: c_int)
     -> c_int;
    pub fn av_append_packet(s: *mut AVIOContext, pkt: *mut avcodec::AVPacket,
                            size: c_int) -> c_int;
    pub fn av_fmt_ctx_get_duration_estimation_method(ctx: *AVFormatContext) ->
     Enum_AVDurationEstimationMethod;
    pub fn avformat_version() -> c_uint;
    pub fn avformat_configuration() -> *c_schar;
    pub fn avformat_license() -> *c_schar;
    pub fn av_register_all();
    pub fn av_register_input_format(format: *mut AVInputFormat);
    pub fn av_register_output_format(format: *mut AVOutputFormat);
    pub fn avformat_network_init() -> c_int;
    pub fn avformat_network_deinit() -> c_int;
    pub fn av_iformat_next(f: *mut AVInputFormat) -> *mut AVInputFormat;
    pub fn av_oformat_next(f: *mut AVOutputFormat) -> *mut AVOutputFormat;
    pub fn avformat_alloc_context() -> *mut AVFormatContext;
    pub fn avformat_free_context(s: *mut AVFormatContext);
    pub fn avformat_get_class() -> *avutil::AVClass;
    pub fn avformat_new_stream(s: *mut AVFormatContext, c: *avcodec::AVCodec) ->
     *mut AVStream;
    pub fn av_new_program(s: *mut AVFormatContext, id: c_int) ->
     *mut AVProgram;
    pub fn avformat_alloc_output_context(format: *c_schar,
                                         oformat: *mut AVOutputFormat,
                                         filename: *c_schar) ->
     *mut AVFormatContext;
    pub fn avformat_alloc_output_context2(ctx: *mut *mut AVFormatContext,
                                          oformat: *mut AVOutputFormat,
                                          format_name: *c_schar,
                                          filename: *c_schar) -> c_int;
    pub fn av_find_input_format(short_name: *c_schar) -> *mut AVInputFormat;
    pub fn av_probe_input_format(pd: *mut AVProbeData, is_opened: c_int) ->
     *mut AVInputFormat;
    pub fn av_probe_input_format2(pd: *mut AVProbeData, is_opened: c_int,
                                  score_max: *mut c_int) ->
     *mut AVInputFormat;
    pub fn av_probe_input_format3(pd: *mut AVProbeData, is_opened: c_int,
                                  score_ret: *mut c_int) ->
     *mut AVInputFormat;
    pub fn av_probe_input_buffer(pb: *mut AVIOContext,
                                 fmt: *mut *mut AVInputFormat,
                                 filename: *c_schar, logctx: *mut c_void,
                                 offset: c_uint, max_probe_size: c_uint) ->
     c_int;
    pub fn avformat_open_input(ps: *mut *mut AVFormatContext,
                               filename: *c_schar, fmt: *mut AVInputFormat,
                               options: *mut *mut avutil::AVDictionary) -> c_int;
    pub fn av_demuxer_open(ic: *mut AVFormatContext) -> c_int;
    pub fn av_find_stream_info(ic: *mut AVFormatContext) -> c_int;
    pub fn avformat_find_stream_info(ic: *mut AVFormatContext,
                                     options: *mut *mut avutil::AVDictionary) ->
     c_int;
    pub fn av_find_program_from_stream(ic: *mut AVFormatContext,
                                       last: *mut AVProgram, s: c_int) ->
     *mut AVProgram;
    pub fn av_find_best_stream(ic: *mut AVFormatContext,
                               _type: avutil::Enum_AVMediaType,
                               wanted_stream_nb: c_int, related_stream: c_int,
                               decoder_ret: *mut *mut avcodec::AVCodec, flags: c_int)
     -> c_int;
    pub fn av_read_packet(s: *mut AVFormatContext, pkt: *mut avcodec::AVPacket) ->
     c_int;
    pub fn av_read_frame(s: *mut AVFormatContext, pkt: *mut avcodec::AVPacket) ->
     c_int;
    pub fn av_seek_frame(s: *mut AVFormatContext, stream_index: c_int,
                         timestamp: int64_t, flags: c_int) -> c_int;
    pub fn avformat_seek_file(s: *mut AVFormatContext, stream_index: c_int,
                              min_ts: int64_t, ts: int64_t, max_ts: int64_t,
                              flags: c_int) -> c_int;
    pub fn av_read_play(s: *mut AVFormatContext) -> c_int;
    pub fn av_read_pause(s: *mut AVFormatContext) -> c_int;
    pub fn av_close_input_file(s: *mut AVFormatContext);
    pub fn avformat_close_input(s: *mut *mut AVFormatContext);
    pub fn av_new_stream(s: *mut AVFormatContext, id: c_int) -> *mut AVStream;
    pub fn av_set_pts_info(s: *mut AVStream, pts_wrap_bits: c_int,
                           pts_num: c_uint, pts_den: c_uint);
    pub fn avformat_write_header(s: *mut AVFormatContext,
                                 options: *mut *mut avutil::AVDictionary) -> c_int;
    pub fn av_write_frame(s: *mut AVFormatContext, pkt: *mut avcodec::AVPacket) ->
     c_int;
    pub fn av_interleaved_write_frame(s: *mut AVFormatContext,
                                      pkt: *mut avcodec::AVPacket) -> c_int;
    pub fn av_interleave_packet_per_dts(s: *mut AVFormatContext,
                                        out: *mut avcodec::AVPacket,
                                        pkt: *mut avcodec::AVPacket, flush: c_int) ->
     c_int;
    pub fn av_write_trailer(s: *mut AVFormatContext) -> c_int;
    pub fn av_guess_format(short_name: *c_schar, filename: *c_schar,
                           mime_type: *c_schar) -> *mut AVOutputFormat;
    pub fn av_guess_codec(fmt: *mut AVOutputFormat, short_name: *c_schar,
                          filename: *c_schar, mime_type: *c_schar,
                          _type: avutil::Enum_AVMediaType) -> avcodec::Enum_AVCodecID;
    pub fn av_get_output_timestamp(s: *mut Struct_AVFormatContext,
                                   stream: c_int, dts: *mut int64_t,
                                   wall: *mut int64_t) -> c_int;
    pub fn av_hex_dump(f: *mut FILE, buf: *uint8_t, size: c_int);
    pub fn av_hex_dump_log(avcl: *mut c_void, level: c_int, buf: *uint8_t,
                           size: c_int);
    pub fn av_pkt_dump2(f: *mut FILE, pkt: *mut avcodec::AVPacket, dump_payload: c_int,
                        st: *mut AVStream);
    pub fn av_pkt_dump_log2(avcl: *mut c_void, level: c_int,
                            pkt: *mut avcodec::AVPacket, dump_payload: c_int,
                            st: *mut AVStream);
    pub fn av_codec_get_id(tags: **Struct_AVCodecTag, tag: c_uint) ->
     avcodec::Enum_AVCodecID;
    pub fn av_codec_get_tag(tags: **Struct_AVCodecTag, id: avcodec::Enum_AVCodecID) ->
     c_uint;
    pub fn av_codec_get_tag2(tags: **Struct_AVCodecTag, id: avcodec::Enum_AVCodecID,
                             tag: *mut c_uint) -> c_int;
    pub fn av_find_default_stream_index(s: *mut AVFormatContext) -> c_int;
    pub fn av_index_search_timestamp(st: *mut AVStream, timestamp: int64_t,
                                     flags: c_int) -> c_int;
    pub fn av_add_index_entry(st: *mut AVStream, pos: int64_t,
                              timestamp: int64_t, size: c_int,
                              distance: c_int, flags: c_int) -> c_int;
    pub fn av_url_split(proto: *mut c_schar, proto_size: c_int,
                        authorization: *mut c_schar,
                        authorization_size: c_int, hostname: *mut c_schar,
                        hostname_size: c_int, port_ptr: *mut c_int,
                        path: *mut c_schar, path_size: c_int, url: *c_schar);
    pub fn av_dump_format(ic: *mut AVFormatContext, index: c_int,
                          url: *c_schar, is_output: c_int);
    pub fn av_get_frame_filename(buf: *mut c_schar, buf_size: c_int,
                                 path: *c_schar, number: c_int) -> c_int;
    pub fn av_filename_number_test(filename: *c_schar) -> c_int;
//    pub fn av_sdp_create(ac: c_void, n_files: c_int, buf: *mut c_schar,
//                         size: c_int) -> c_int;
    pub fn av_sdp_create(ac: *mut *mut Struct_AVFormatContext, n_files: c_int, buf: *mut c_schar,
                         size: c_int) -> c_int;
    pub fn av_match_ext(filename: *c_schar, extensions: *c_schar) -> c_int;
    pub fn avformat_query_codec(ofmt: *mut AVOutputFormat,
                                codec_id: avcodec::Enum_AVCodecID,
                                std_compliance: c_int) -> c_int;
    pub fn avformat_get_riff_video_tags() -> *Struct_AVCodecTag;
    pub fn avformat_get_riff_audio_tags() -> *Struct_AVCodecTag;
    pub fn av_guess_sample_aspect_ratio(format: *mut AVFormatContext,
                                        stream: *mut AVStream,
                                        frame: *mut avcodec::AVFrame) -> avutil::AVRational;
    pub fn avformat_match_stream_specifier(s: *mut AVFormatContext,
                                           st: *mut AVStream, spec: *c_schar)
     -> c_int;
    pub fn avformat_queue_attached_pictures(s: *mut AVFormatContext);
}

pub fn version() -> uint{
    unsafe {
        avformat_version() as uint
    }
}
pub fn license() -> ~str {
    unsafe {
        std::str::raw::from_c_str(avformat_license())
    }
}
