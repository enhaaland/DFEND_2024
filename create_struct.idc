
static main() {

    auto offset_address = 0x0000000000605108;
    auto i;

    for ( i = 0; i < 264; i++) {
        make_array( offset_address, 71);
        offset_address = offset_address + 71;
    }
}