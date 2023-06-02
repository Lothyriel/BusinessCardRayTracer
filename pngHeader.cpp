
void writePngHeader(int width, int height) {
    // PNG file signature
    printf("\x89\x50\x4E\x47\x0D\x0A\x1A\x0A");

    // PNG IHDR chunk (image header)
    unsigned char ihdr[13] = {
        (width >> 24) & 0xFF,
        (width >> 16) & 0xFF,
        (width >> 8) & 0xFF,
        width & 0xFF,
        (height >> 24) & 0xFF,
        (height >> 16) & 0xFF,
        (height >> 8) & 0xFF,
        height & 0xFF,
        8, // Bits per channel (8 bits for red, green, blue channels)
        2, // Color type: RGB
        0, // Compression method: none
        0, // Filter method: adaptive
        0  // Interlace method: no interlace
    };

    auto a = reinterpret_cast<const char*>(ihdr);
    //printf(, sizeof(ihdr));

    // Calculate CRC32 checksum
    auto crc32 = [](const unsigned char* buf, size_t size) {
        unsigned int crc = 0xFFFFFFFF;
        static unsigned int crc_table[256];

        if (!*crc_table)
            for (unsigned int i = 0; i < 256; i++) {
                unsigned int c = i;
                for (int j = 0; j < 8; j++)
                    c = (c & 1) ? (0xEDB88320 ^ (c >> 1)) : (c >> 1);
                crc_table[i] = c;
            }

        while (size--)
            crc = crc_table[(crc ^ *buf++) & 0xFF] ^ (crc >> 8);

        return crc ^ 0xFFFFFFFF;
    };

    // Calculate and write CRC32 checksum for IHDR chunk
    unsigned int crc = crc32(ihdr, sizeof(ihdr));
    printf(reinterpret_cast<const char*>(&crc), sizeof(crc));
}