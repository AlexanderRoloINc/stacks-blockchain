;; the .costs contract

;; Helper Functions

;; Return a Cost Specification with just a runtime cost
(define-private (runtime (r uint))
    {
        runtime: r,
        write_length: 0,
        write_count: 0,
        read_count: 0,
        read_length: 0,
    })

;; Linear cost-assessment function
(define-private (linear (n uint) (a uint) (b uint))
    (+ (* a n) b))

;; TODO: fix this once log is available
;; LogN cost-assessment function
(define-private (logn (n uint) (a uint) (b uint))
    (+ (* a n) b))

;; TODO: fix this once log is available
;; NLogN cost-assessment function
(define-private (nlogn (n uint) (a uint) (b uint))
    (+ (* a n) b))


;; Cost Functions

(define-read-only (cost_analysis_type_annotate (n uint))
    (runtime (linear n u1 u1)))

(define-read-only (cost_analysis_type_check (n uint))
    (runtime (linear n u1 u1)))

(define-read-only (cost_analysis_type_lookup (n uint))
    (runtime (linear n u1 u1)))

(define-read-only (cost_analysis_visit) (runtime 1))

(define-read-only (cost_analysis_iterable_func) (runtime 1))

(define-read-only (cost_analysis_option_cons) (runtime 1))

(define-read-only (cost_analysis_option_check) (runtime 1))

(define-read-only (cost_analysis_bind_name (n uint))
    (runtime (linear n u1 u1)))

(define-read-only (cost_analysis_list_items_check (n uint))
    (runtime (linear n u1 u1)))

(define-read-only (cost_analysis_check_tuple_get (n uint))
    (runtime (logn n u1 u1)))

(define-read-only (cost_analysis_check_tuple_cons (n uint))
    (runtime (nlogn n u1 u1)))

(define-read-only (cost_analysis_tuple_items_check (n uint))
    (runtime (linear n u1 u1)))

(define-read-only (cost_analysis_check_let (n uint))
    (runtime (linear n u1 u1)))

(define-read-only (cost_analysis_lookup_function_types (n uint))
    (runtime (linear n u1 u1)))

(define-read-only (cost_analysis_lookup_variable_const) (runtime 1))

(define-read-only (cost_analysis_lookup_variable_depth (n uint))
    (runtime (nlogn n u1 u1)))

(define-read-only (cost_ast_parse (n uint))
    (runtime (linear n u1 u1)))

(define-read-only (cost_ast_cycle_detection (n uint))
    (runtime (linear n u1 u1)))

(define-read-only (cost_analysis_storage (n uint))
    {
        runtime: (linear n u1 u1),
        write_length: (linear n u1 u1),
        write_count: u1,
        read_count: u1,
        read_length: u1
    })

(define-read-only (cost_analysis_use_trait_entry (n uint))
    {
        runtime: (linear n u1 u1),
        write_length: (linear n u1 u1),
        write_count: 0,
        read_count: 1,
        read_length: (linear n u1 u1)
    })

(define-read-only (cost_analysis_get_function_entry (n uint))
    {
        runtime: (linear n u1 u1),
        write_length: 0,
        write_count: 0,
        read_count: 1,
        read_length: (linear n u1 u1)
    })

(define-read-only (cost_analysis_fetch_contract_entry (n uint))
    {
        runtime: (linear n u1 u1),
        write_length: 0,
        write_count: 0,
        read_count: 1,
        read_length: (linear n u1 u1)
    })

(define-read-only (cost_lookup_variable_depth (n uint))
    (runtime (linear n u1 u1)))

(define-read-only (cost_lookup_variable_size (n uint))
    (runtime (linear n u1 u0)))

(define-read-only (cost_lookup_function) (runtime 1))

(define-read-only (cost_bind_name) (runtime 1))

(define-read-only (cost_inner_type_check_cost (n uint))
    (runtime (linear n u1 u1)))

(define-read-only (cost_user_function_application (n uint))
    (runtime (linear n u1 u1)))

(define-read-only (cost_let (n uint))
    (runtime (linear n u1 u1)))

(define-read-only (cost_if) (runtime 1))

(define-read-only (cost_asserts) (runtime 1))

(define-read-only (cost_map) (runtime 1))

(define-read-only (cost_filter) (runtime 1))

(define-read-only (cost_len) (runtime 1))

(define-read-only (cost_fold) (runtime 1))

(define-read-only (cost_list_cons (n uint))
    (runtime (linear n u1 u1)))

(define-read-only (cost_type_parse_step) (runtime 1))

(define-read-only (cost_data_hash_cost (n uint))
    (runtime (linear n u1 u1)))

(define-read-only (cost_tuple_get (n uint))
    (runtime (nLogN n u1 u1)))

(define-read-only (cost_tuple_cons (n uint))
    (runtime (nLogN n u1 u1)))

(define-read-only (cost_add (n uint))
    (runtime (linear n u1 u1)))

(define-read-only (cost_sub (n uint))
    (runtime (linear n u1 u1)))

(define-read-only (cost_mul (n uint))
    (runtime (linear n u1 u1)))

(define-read-only (cost_div (n uint))
    (runtime (linear n u1 u1)))

(define-read-only (cost_geq) (runtime 1))

(define-read-only (cost_leq) (runtime 1))

(define-read-only (cost_le ) (runtime 1))

(define-read-only (cost_ge ) (runtime 1))

(define-read-only (cost_int_cast) (runtime 1))

(define-read-only (cost_mod) (runtime 1))

(define-read-only (cost_pow) (runtime 1))

(define-read-only (cost_sqrti) (runtime 1))

(define-read-only (cost_xor) (runtime 1))

(define-read-only (cost_not) (runtime 1))

(define-read-only (cost_eq (n uint))
    (runtime (linear n u1 u1)))

(define-read-only (cost_begin) (runtime 1))

(define-read-only (cost_hash160) (runtime 1))

(define-read-only (cost_sha256) (runtime 1))

(define-read-only (cost_sha512) (runtime 1))

(define-read-only (cost_sha512t256) (runtime 1))

(define-read-only (cost_keccak256) (runtime 1))

(define-read-only (cost_secp256k1recover) (runtime 1))

(define-read-only (cost_secp256k1verify) (runtime 1))

(define-read-only (cost_print (n uint))
    (runtime (linear n u1 u1)))

(define-read-only (cost_some_cons) (runtime 1))

(define-read-only (cost_ok_cons) (runtime 1))

(define-read-only (cost_err_cons) (runtime 1))

(define-read-only (cost_default_to) (runtime 1))

(define-read-only (cost_unwrap_ret) (runtime 1))

(define-read-only (cost_unwrap_err_or_ret) (runtime 1))

(define-read-only (cost_is_okay) (runtime 1))

(define-read-only (cost_is_none) (runtime 1))

(define-read-only (cost_is_err) (runtime 1))

(define-read-only (cost_is_some) (runtime 1))

(define-read-only (cost_unwrap) (runtime 1))

(define-read-only (cost_unwrap_err) (runtime 1))

(define-read-only (cost_try_ret) (runtime 1))

(define-read-only (cost_match) (runtime 1))

(define-read-only (cost_or (n uint))
    (runtime (linear n u1 u1)))

(define-read-only (cost_and (n uint))
    (runtime (linear n u1 u1)))

(define-read-only (cost_append (n uint))
    (runtime (linear n u1 u1)))

(define-read-only (cost_concat (n uint))
    (runtime (linear n u1 u1)))

(define-read-only (cost_as_max_len) (runtime 1))

(define-read-only (cost_contract_call) (runtime 1))

(define-read-only (cost_contract_of) (runtime 1))

(define-read-only (cost_principal_of) (runtime 1))

(define-read-only (cost_at_block)
    {
        runtime: 1,
        write_length: 0,
        write_count: 0,
        read_count: 1,
        read_length: 1
    })

(define-read-only (cost_load_contract (n uint))
    {
        runtime: (linear n u1 u1),
        write_length: 0,
        write_count: 0,
        read_count: 1,
        read_length: (linear n u1 u1)
    })

(define-read-only (cost_create_map (n uint))
    {
        runtime: (linear n u1 u1),
        write_length: (linear n u1 u1),
        write_count: 1,
        read_count: 0,
        read_length: 0
    })

(define-read-only (cost_create_var (n uint))
    {
        runtime: (linear n u1 u1),
        write_length: (linear n u1 u1),
        write_count: 2,
        read_count: 0,
        read_length: 0
    })

(define-read-only (cost_create_nft (n uint))
    {
        runtime: (linear n u1 u1),
        write_length: (linear n u1 u1),
        write_count: 1,
        read_count: 0,
        read_length: 0
    })

(define-read-only (cost_create_ft)
    {
        runtime: 1,
        write_length: 1,
        write_count: 2,
        read_count: 0,
        read_length: 0
    })

(define-read-only (cost_fetch_entry (n uint))
    {
        runtime: (linear n u1 u1),
        write_length: 0,
        write_count: 0,
        read_count: 1,
        read_length: (linear n u1 u1)
    })

(define-read-only (cost_set_entry (n uint))
    {
        runtime: (linear n u1 u1),
        write_length: (linear n u1 u1),
        write_count: 1,
        read_count: 1,
        read_length: 0
    })

(define-read-only (cost_fetch_var (n uint))
    {
        runtime: (linear n u1 u1),
        write_length: 0,
        write_count: 0,
        read_count: 1,
        read_length: (linear n u1 u1)
    })

(define-read-only (cost_set_var (n uint))
    {
        runtime: (linear n u1 u1),
        write_length: (linear n u1 u1),
        write_count: 1,
        read_count: 1,
        read_length: 0
    })

(define-read-only (cost_contract_storage (n uint))
    {
        runtime: (linear n u1 u1),
        write_length: (linear n u1 u1),
        write_count: 1,
        read_count: 0,
        read_length: 0
    })

(define-read-only (cost_block_info)
    {
        runtime: 1,
        write_length: 0,
        write_count: 0,
        read_count: 1,
        read_length: 1
    })

(define-read-only (cost_stx_balance)
    {
        runtime: 1,
        write_length: 0,
        write_count: 0,
        read_count: 1,
        read_length: 1
    })

(define-read-only (cost_stx_transfer)
    {
        runtime: 1,
        write_length: 1,
        write_count: 1,
        read_count: 1,
        read_length: 1
    })

(define-read-only (cost_ft_mint)
    {
        runtime: 1,
        write_length: 1,
        write_count: 2,
        read_count: 2,
        read_length: 1
    })

(define-read-only (cost_ft_transfer)
    {
        runtime: 1,
        write_length: 1,
        write_count: 2,
        read_count: 2,
        read_length: 1
    })

(define-read-only (cost_ft_balance)
    {
        runtime: 1,
        write_length: 0,
        write_count: 0,
        read_count: 1,
        read_length: 1
    })

(define-read-only (cost_nft_mint (n uint))
    {
        runtime: (linear n u1 u1),
        write_length: 1,
        write_count: 1,
        read_count: 1,
        read_length: 1
    })

(define-read-only (cost_nft_transfer (n uint))
    {
        runtime: (linear n u1 u1),
        write_length: 1,
        write_count: 1,
        read_count: 1,
        read_length: 1
    })

(define-read-only (cost_nft_owner (n uint))
    {
        runtime: (linear n u1 u1),
        write_length: 0,
        write_count: 0,
        read_count: 1,
        read_length: 1
    })
