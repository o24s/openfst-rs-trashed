#include "cpp/sparse-power-weight.h"

namespace fst_rust {
namespace ffi {

thread_local std::vector<SparseTupleElementF32> tls_sparse_buffer_f32;
thread_local std::vector<SparseTupleElementF64> tls_sparse_buffer_f64;

}
}
