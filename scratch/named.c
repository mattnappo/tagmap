typedef int empty;

enum NamedTag { A, B };

struct Named {
    enum NamedTag variant;
    union {
        struct {
            char *x;
            int y;
        } a;
        empty b;
    };
};
