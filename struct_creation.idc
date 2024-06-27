static main() {

    auto offset_address = 0x0000000000605108;
    auto i;

    for ( i = 0; i < 264; i++) {
        create_struct( offset_address, 71 , "important_chunk");
        offset_address = offset_address + 71;
    }
}