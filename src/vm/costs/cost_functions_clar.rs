use std::collections::HashMap;
use std::string::ToString;
use super::{ClarityCostFunctionReference, SimpleCostSpecification, TypeCheckCost};
use super::CostFunctions::{Linear, Constant, NLogN, LogN};
use vm::types::{Value, QualifiedContractIdentifier, StandardPrincipalData};
use vm::contexts::{OwnedEnvironment, Environment};
use vm::database::MemoryBackingStore;
use chainstate::stacks::db::StacksChainState;
use chainstate::stacks::boot::boot_code_id;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

#[derive(Debug, Display, EnumIter, Deserialize, Serialize, Clone, PartialEq, Eq, Hash)]
pub enum ClarityCostFunctionName {
    ANALYSIS_TYPE_ANNOTATE,
    ANALYSIS_TYPE_CHECK,
    ANALYSIS_TYPE_LOOKUP,
    ANALYSIS_VISIT,
    ANALYSIS_ITERABLE_FUNC,
    ANALYSIS_OPTION_CONS,
    ANALYSIS_OPTION_CHECK,
    ANALYSIS_BIND_NAME,
    ANALYSIS_LIST_ITEMS_CHECK,
    ANALYSIS_CHECK_TUPLE_GET,
    ANALYSIS_CHECK_TUPLE_CONS,
    ANALYSIS_TUPLE_ITEMS_CHECK,
    ANALYSIS_CHECK_LET,
    ANALYSIS_LOOKUP_FUNCTION,
    ANALYSIS_LOOKUP_FUNCTION_TYPES,
    ANALYSIS_LOOKUP_VARIABLE_CONST,
    ANALYSIS_LOOKUP_VARIABLE_DEPTH,
    AST_PARSE,
    AST_CYCLE_DETECTION,
    ANALYSIS_STORAGE,
    ANALYSIS_USE_TRAIT_ENTRY,
    ANALYSIS_GET_FUNCTION_ENTRY,
    ANALYSIS_FETCH_CONTRACT_ENTRY,
    LOOKUP_VARIABLE_DEPTH,
    LOOKUP_VARIABLE_SIZE,
    LOOKUP_FUNCTION,
    BIND_NAME,
    INNER_TYPE_CHECK_COST,
    USER_FUNCTION_APPLICATION,
    LET,
    IF,
    ASSERTS,
    MAP,
    FILTER,
    LEN,
    FOLD,
    LIST_CONS,
    TYPE_PARSE_STEP,
    DATA_HASH_COST,
    TUPLE_GET,
    TUPLE_CONS,
    ADD,
    SUB,
    MUL,
    DIV,
    GEQ,
    LEQ,
    LE,
    GE,
    INT_CAST,
    MOD,
    POW,
    SQRTI,
    XOR,
    NOT,
    EQ,
    BEGIN,
    HASH160,
    SHA256,
    SHA512,
    SHA512T256,
    KECCAK256,
    SECP256K1RECOVER,
    SECP256K1VERIFY,
    PRINT,
    SOME_CONS,
    OK_CONS,
    ERR_CONS,
    DEFAULT_TO,
    UNWRAP_RET,
    UNWRAP_ERR_OR_RET,
    IS_OKAY,
    IS_NONE,
    IS_ERR,
    IS_SOME,
    UNWRAP,
    UNWRAP_ERR,
    TRY_RET,
    MATCH,
    OR,
    AND,
    APPEND,
    CONCAT,
    AS_MAX_LEN,
    CONTRACT_CALL,
    CONTRACT_OF,
    PRINCIPAL_OF,
    AT_BLOCK,
    LOAD_CONTRACT,
    CREATE_MAP,
    CREATE_VAR,
    CREATE_NFT,
    CREATE_FT,
    FETCH_ENTRY,
    SET_ENTRY,
    FETCH_VAR,
    SET_VAR,
    CONTRACT_STORAGE,
    BLOCK_INFO,
    STX_BALANCE,
    STX_TRANSFER,
    FT_MINT,
    FT_TRANSFER,
    FT_BALANCE,
    NFT_MINT,
    NFT_TRANSFER,
    NFT_OWNER,
}

fn boot_function(function_name: &'static str) -> ClarityCostFunctionReference {
    ClarityCostFunctionReference {
        contract_id: boot_code_id("costs"),
        function_name,
    }
}

lazy_static! {
    static ref COSTS: HashMap<ClarityCostFunctionName, ClarityCostFunctionReference> = {
        let mut m = HashMap::new();

        for name in ClarityCostFunctionName::iter() {
            let function_name: &'static str = Box::leak(
                format!("cost_{}", name.clone().to_string().to_lowercase()).into_boxed_str());
            m.insert(name.clone(), boot_function(function_name));
        }
        m
    };
}

fn eval_contract_cost(
    mut env: Environment,
    cost_function: ClarityCostFunctionReference,
    input_size: u64) -> Result<Value, &'static str> {

    let eval_result = env.eval_read_only(
        &cost_function.contract_id,
        format!("({} u{})", cost_function.function_name, input_size).as_str());

    match eval_result {
        Ok(Value::Tuple(data)) => {
            match data.data_map.get("runtime") {
                Some(runtime) => Ok(runtime.clone()),
                None => Err("err")
            }
        }, 
        Ok(_) => Err("cost function result not a Tuple"),
        Err(_) => Err("error evaluating result of cost function"),
    }
}

#[test]
fn test_eval_contract_cost() {
    // setup env
    let mut marf = MemoryBackingStore::new();
    let mut owned_env = OwnedEnvironment::new(marf.as_clarity_db());

    // deploy boot cost contract
    owned_env.initialize_contract(boot_code_id("costs"), std::include_str!("../../chainstate/stacks/boot/costs.clar")).unwrap();
    let env = owned_env.get_exec_environment(None);

    let cost = eval_contract_cost(env, COSTS.get(&ClarityCostFunctionName::ANALYSIS_TYPE_ANNOTATE).unwrap().clone(), 10);
    assert!(cost == Ok(Value::UInt(11)));
}