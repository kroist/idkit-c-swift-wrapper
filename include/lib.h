#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct Proof {
  const char *proof;
  const char *merkle_root;
  const char *nullifier_hash;
  const char *credential_type;
} Proof;

struct Proof get_proof(void);
