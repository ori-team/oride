#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int safe_copy_message(char* dest, size_t dest_cap, const char* src) {
    if (dest == NULL || src == NULL || dest_cap == 0) {
        return -1;
    }
    int written = snprintf(dest, dest_cap, "%s", src);
    if (written < 0 || (size_t)written >= dest_cap) {
        return -2; // Truncation or formatting error
    }
    return 0; // Success
}

int main(void) {
    char buffer[64];
    int res = safe_copy_message(buffer, sizeof(buffer), "Safe systems message");
    if (res == 0) {
        printf("Buffer content: %s\n", buffer);
    }
    return res;
}
