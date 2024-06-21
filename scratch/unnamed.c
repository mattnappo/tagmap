#include <assert.h>
#include <string.h>

enum UnnamedTag {
    A,
    B
};

struct Unnamed {
    enum UnnamedTag variant;
    union {
        struct {
            char *t0;
            int   t1;
        } a;
        long long b;
    };
};

int main() {

    struct Unnamed u = {
        .variant = A,
        .a = { "abc", 12 }
    };

    assert(strcmp(u.a.t0, "abc") == 0);
    assert(u.a.t1 == 12);

    return 0;
}
