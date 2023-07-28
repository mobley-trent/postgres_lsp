use pg_query::{Node, NodeRef};

/// Returns the array children nodes
pub fn get_flat_nodes<'a>(node: &'a NodeRef) -> Vec<NodeRef<'a>> {
    match node {
        NodeRef::CreateStmt(n) => {
            let mut nodes: Vec<NodeRef> = vec![node.clone()];
            nodes.append(
                &mut n
                    .table_elts
                    .iter()
                    .map(|x| x.node.as_ref().unwrap().to_ref())
                    .collect(),
            );
            nodes.append(
                &mut n
                    .options
                    .iter()
                    .map(|x| x.node.as_ref().unwrap().to_ref())
                    .collect(),
            );
            nodes
        }
        n => vec![n.clone()],
    }
}

/// Gets the position value for a pg_query node
///
/// This can mostly be generated by just returning `node.location` if the type has the property,
/// but there are some exceptions where the location on the node itself is not leftmost position, e.g. for `AExpr`.
pub fn get_position_for_pg_query_node(node: &NodeRef) -> i32 {
    match node {
        NodeRef::ResTarget(n) => n.location,
        NodeRef::AExpr(n) => get_position_for_pg_query_node(
            &n.lexpr.as_ref().unwrap().node.as_ref().unwrap().to_ref(),
        ),
        NodeRef::RangeVar(n) => n.location,
        NodeRef::ColumnRef(n) => n.location,
        NodeRef::AConst(n) => n.location,
        NodeRef::Alias(n) => todo!("Alias"),
        NodeRef::TableFunc(n) => n.location,
        NodeRef::Expr(n) => todo!("Expr"),
        NodeRef::Var(n) => n.location,
        NodeRef::Param(n) => n.location,
        NodeRef::Aggref(n) => n.location,
        NodeRef::GroupingFunc(n) => n.location,
        NodeRef::WindowFunc(n) => n.location,
        NodeRef::SubscriptingRef(n) => todo!("SubscriptingRef"),
        NodeRef::FuncExpr(n) => n.location,
        NodeRef::NamedArgExpr(n) => n.location,
        NodeRef::OpExpr(n) => n.location,
        NodeRef::DistinctExpr(n) => n.location,
        NodeRef::NullIfExpr(n) => n.location,
        NodeRef::ScalarArrayOpExpr(n) => n.location,
        NodeRef::BoolExpr(n) => n.location,
        NodeRef::SubLink(n) => n.location,
        NodeRef::SubPlan(n) => {
            get_position_for_pg_query_node(&n.xpr.as_ref().unwrap().node.as_ref().unwrap().to_ref())
        }
        NodeRef::AlternativeSubPlan(n) => todo!("AlternativeSubPlan"),
        NodeRef::FieldSelect(n) => todo!("FieldSelect"),
        NodeRef::FieldStore(n) => todo!("FieldStore"),
        NodeRef::RelabelType(n) => n.location,
        NodeRef::CoerceViaIo(n) => n.location,
        NodeRef::ArrayCoerceExpr(n) => n.location,
        NodeRef::ConvertRowtypeExpr(n) => n.location,
        NodeRef::CollateExpr(n) => n.location,
        NodeRef::CaseExpr(n) => n.location,
        NodeRef::CaseWhen(n) => n.location,
        NodeRef::CaseTestExpr(n) => todo!("CaseTestExpr"),
        NodeRef::ArrayExpr(n) => n.location,
        NodeRef::RowExpr(n) => n.location,
        NodeRef::RowCompareExpr(n) => todo!("RowCompareExpr"),
        NodeRef::CoalesceExpr(n) => n.location,
        NodeRef::MinMaxExpr(n) => n.location,
        NodeRef::SqlvalueFunction(n) => n.location,
        NodeRef::XmlExpr(n) => n.location,
        NodeRef::NullTest(n) => n.location,
        NodeRef::BooleanTest(n) => n.location,
        NodeRef::CoerceToDomain(n) => n.location,
        NodeRef::CoerceToDomainValue(n) => n.location,
        NodeRef::SetToDefault(n) => n.location,
        NodeRef::CurrentOfExpr(n) => todo!("CurrentOfExpr"),
        NodeRef::NextValueExpr(_) => todo!("NextValueExpr"),
        NodeRef::InferenceElem(_) => todo!("InferenceElem"),
        NodeRef::TargetEntry(_) => todo!("TargetEntry"),
        NodeRef::RangeTblRef(_) => todo!("RangeTblRef"),
        NodeRef::JoinExpr(_) => todo!("JoinExpr"),
        NodeRef::FromExpr(_) => todo!("FromExpr"),
        NodeRef::OnConflictExpr(_) => todo!("OnConflictExpr"),
        NodeRef::IntoClause(_) => todo!("IntoClause"),
        NodeRef::RawStmt(_) => todo!("RawStmt"),
        NodeRef::Query(_) => todo!("Query"),
        NodeRef::InsertStmt(_) => todo!("InsertStmt"),
        NodeRef::DeleteStmt(_) => todo!("DeleteStmt"),
        NodeRef::UpdateStmt(_) => todo!("UpdateStmt"),
        NodeRef::SelectStmt(_) => todo!("SelectStmt"),
        NodeRef::AlterTableStmt(_) => -1,
        NodeRef::AlterTableCmd(_) => todo!("AlterTableCmd"),
        NodeRef::AlterDomainStmt(_) => todo!("AlterDomainStmt"),
        NodeRef::SetOperationStmt(_) => todo!("SetOperationStmt"),
        NodeRef::GrantStmt(_) => -1,
        NodeRef::GrantRoleStmt(_) => todo!("GrantRoleStmt"),
        NodeRef::AlterDefaultPrivilegesStmt(_) => todo!("AlterDefaultPrivilegesStmt"),
        NodeRef::ClosePortalStmt(_) => todo!("ClosePortalStmt"),
        NodeRef::ClusterStmt(_) => todo!("ClusterStmt"),
        NodeRef::CopyStmt(_) => todo!("CopyStmt"),
        NodeRef::CreateStmt(_) => -1,
        NodeRef::DefineStmt(_) => todo!("DefineStmt"),
        NodeRef::DropStmt(_) => todo!("DropStmt"),
        NodeRef::TruncateStmt(_) => todo!("TruncateStmt"),
        NodeRef::CommentStmt(_) => todo!("CommentStmt"),
        NodeRef::FetchStmt(_) => todo!("FetchStmt"),
        NodeRef::IndexStmt(_) => todo!("IndexStmt"),
        NodeRef::CreateFunctionStmt(_) => todo!("CreateFunctionStmt"),
        NodeRef::AlterFunctionStmt(_) => todo!("AlterFunctionStmt"),
        NodeRef::DoStmt(_) => todo!("DoStmt"),
        NodeRef::RenameStmt(_) => todo!("RenameStmt"),
        NodeRef::RuleStmt(_) => todo!("RuleStmt"),
        NodeRef::NotifyStmt(_) => todo!("NotifyStmt"),
        NodeRef::ListenStmt(_) => todo!("ListenStmt"),
        NodeRef::UnlistenStmt(_) => todo!("UnlistenStmt"),
        NodeRef::TransactionStmt(_) => todo!("TransactionStmt"),
        NodeRef::ViewStmt(_) => todo!("ViewStmt"),
        NodeRef::LoadStmt(_) => todo!("LoadStmt"),
        NodeRef::CreateDomainStmt(_) => todo!("CreateDomainStmt"),
        NodeRef::CreatedbStmt(_) => todo!("CreatedbStmt"),
        NodeRef::DropdbStmt(_) => todo!("DropdbStmt"),
        NodeRef::VacuumStmt(_) => todo!("VacuumStmt"),
        NodeRef::ExplainStmt(_) => todo!("ExplainStmt"),
        NodeRef::CreateTableAsStmt(_) => todo!("CreateTableAsStmt"),
        NodeRef::CreateSeqStmt(_) => todo!("CreateSeqStmt"),
        NodeRef::AlterSeqStmt(_) => todo!("AlterSeqStmt"),
        NodeRef::VariableSetStmt(_) => todo!("VariableSetStmt"),
        NodeRef::VariableShowStmt(_) => todo!("VariableShowStmt"),
        NodeRef::DiscardStmt(_) => todo!("DiscardStmt"),
        NodeRef::CreateTrigStmt(_) => todo!("CreateTrigStmt"),
        NodeRef::CreatePlangStmt(_) => todo!("CreatePlangStmt"),
        NodeRef::CreateRoleStmt(_) => todo!("CreateRoleStmt"),
        NodeRef::AlterRoleStmt(_) => todo!("AlterRoleStmt"),
        NodeRef::DropRoleStmt(_) => todo!("DropRoleStmt"),
        NodeRef::LockStmt(_) => todo!("LockStmt"),
        NodeRef::ConstraintsSetStmt(_) => todo!("ConstraintsSetStmt"),
        NodeRef::ReindexStmt(_) => todo!("ReindexStmt"),
        NodeRef::CheckPointStmt(_) => todo!("CheckPointStmt"),
        NodeRef::CreateSchemaStmt(_) => todo!("CreateSchemaStmt"),
        NodeRef::AlterDatabaseStmt(_) => todo!("AlterDatabaseStmt"),
        NodeRef::AlterDatabaseSetStmt(_) => todo!("AlterDatabaseSetStmt"),
        NodeRef::AlterRoleSetStmt(_) => todo!("AlterRoleSetStmt"),
        NodeRef::CreateConversionStmt(_) => todo!("CreateConversionStmt"),
        NodeRef::CreateCastStmt(_) => todo!("CreateCastStmt"),
        NodeRef::CreateOpClassStmt(_) => todo!("CreateOpClassStmt"),
        NodeRef::CreateOpFamilyStmt(_) => todo!("CreateOpFamilyStmt"),
        NodeRef::AlterOpFamilyStmt(_) => todo!("AlterOpFamilyStmt"),
        NodeRef::PrepareStmt(_) => todo!("PrepareStmt"),
        NodeRef::ExecuteStmt(_) => todo!("ExecuteStmt"),
        NodeRef::DeallocateStmt(_) => todo!("DeallocateStmt"),
        NodeRef::DeclareCursorStmt(_) => todo!("DeclareCursorStmt"),
        NodeRef::CreateTableSpaceStmt(_) => todo!("CreateTableSpaceStmt"),
        NodeRef::DropTableSpaceStmt(_) => todo!("DropTableSpaceStmt"),
        NodeRef::AlterObjectDependsStmt(_) => todo!("AlterObjectDependsStmt"),
        NodeRef::AlterObjectSchemaStmt(_) => todo!("AlterObjectSchemaStmt"),
        NodeRef::AlterOwnerStmt(_) => todo!("AlterOwnerStmt"),
        NodeRef::AlterOperatorStmt(_) => todo!("AlterOperatorStmt"),
        NodeRef::AlterTypeStmt(_) => todo!("AlterTypeStmt"),
        NodeRef::DropOwnedStmt(_) => todo!("DropOwnedStmt"),
        NodeRef::ReassignOwnedStmt(_) => todo!("ReassignOwnedStmt"),
        NodeRef::CompositeTypeStmt(_) => todo!("CompositeTypeStmt"),
        NodeRef::CreateEnumStmt(_) => todo!("CreateEnumStmt"),
        NodeRef::CreateRangeStmt(_) => todo!("CreateRangeStmt"),
        NodeRef::AlterEnumStmt(_) => todo!("AlterEnumStmt"),
        NodeRef::AlterTsdictionaryStmt(_) => todo!("AlterTsdictionaryStmt"),
        NodeRef::AlterTsconfigurationStmt(_) => todo!("AlterTsconfigurationStmt"),
        NodeRef::CreateFdwStmt(_) => todo!("CreateFdwStmt"),
        NodeRef::AlterFdwStmt(_) => todo!("AlterFdwStmt"),
        NodeRef::CreateForeignServerStmt(_) => todo!("CreateForeignServerStmt"),
        NodeRef::AlterForeignServerStmt(_) => todo!("AlterForeignServerStmt"),
        NodeRef::CreateUserMappingStmt(_) => todo!("CreateUserMappingStmt"),
        NodeRef::AlterUserMappingStmt(_) => todo!("AlterUserMappingStmt"),
        NodeRef::DropUserMappingStmt(_) => todo!("DropUserMappingStmt"),
        NodeRef::AlterTableSpaceOptionsStmt(_) => todo!("AlterTableSpaceOptionsStmt"),
        NodeRef::AlterTableMoveAllStmt(_) => todo!("AlterTableMoveAllStmt"),
        NodeRef::SecLabelStmt(_) => todo!("SecLabelStmt"),
        NodeRef::CreateForeignTableStmt(_) => todo!("CreateForeignTableStmt"),
        NodeRef::ImportForeignSchemaStmt(_) => todo!("ImportForeignSchemaStmt"),
        NodeRef::CreateExtensionStmt(_) => todo!("CreateExtensionStmt"),
        NodeRef::AlterExtensionStmt(_) => todo!("AlterExtensionStmt"),
        NodeRef::AlterExtensionContentsStmt(_) => todo!("AlterExtensionContentsStmt"),
        NodeRef::CreateEventTrigStmt(_) => todo!("CreateEventTrigStmt"),
        NodeRef::AlterEventTrigStmt(_) => todo!("AlterEventTrigStmt"),
        NodeRef::RefreshMatViewStmt(_) => todo!("RefreshMatViewStmt"),
        NodeRef::ReplicaIdentityStmt(_) => todo!("ReplicaIdentityStmt"),
        NodeRef::AlterSystemStmt(_) => todo!("AlterSystemStmt"),
        NodeRef::CreatePolicyStmt(_) => todo!("CreatePolicyStmt"),
        NodeRef::AlterPolicyStmt(_) => todo!("AlterPolicyStmt"),
        NodeRef::CreateTransformStmt(_) => todo!("CreateTransformStmt"),
        NodeRef::CreateAmStmt(_) => todo!("CreateAmStmt"),
        NodeRef::CreatePublicationStmt(_) => todo!("CreatePublicationStmt"),
        NodeRef::AlterPublicationStmt(_) => todo!("AlterPublicationStmt"),
        NodeRef::CreateSubscriptionStmt(_) => todo!("CreateSubscriptionStmt"),
        NodeRef::AlterSubscriptionStmt(_) => todo!("AlterSubscriptionStmt"),
        NodeRef::DropSubscriptionStmt(_) => todo!("DropSubscriptionStmt"),
        NodeRef::CreateStatsStmt(_) => todo!("CreateStatsStmt"),
        NodeRef::AlterCollationStmt(_) => todo!("AlterCollationStmt"),
        NodeRef::CallStmt(_) => todo!("CallStmt"),
        NodeRef::AlterStatsStmt(_) => todo!("AlterStatsStmt"),
        NodeRef::ParamRef(n) => n.location,
        NodeRef::FuncCall(n) => n.location,
        NodeRef::AStar(n) => todo!("AStar"),
        NodeRef::AIndices(n) => todo!("AIndices"),
        NodeRef::AIndirection(n) => todo!("AIndirection"),
        NodeRef::AArrayExpr(n) => n.location,
        NodeRef::MultiAssignRef(n) => todo!("MultiAssignRef"),
        NodeRef::TypeCast(n) => n.location,
        NodeRef::CollateClause(n) => n.location,
        NodeRef::SortBy(n) => n.location,
        NodeRef::WindowDef(n) => n.location,
        NodeRef::RangeSubselect(n) => todo!("RangeSubselect"),
        NodeRef::RangeFunction(n) => todo!("RangeFunction"),
        NodeRef::RangeTableSample(n) => n.location,
        NodeRef::RangeTableFunc(n) => n.location,
        NodeRef::RangeTableFuncCol(n) => n.location,
        NodeRef::TypeName(n) => n.location,
        NodeRef::ColumnDef(n) => n.location,
        NodeRef::IndexElem(n) => todo!("IndexElem"),
        NodeRef::Constraint(n) => n.location,
        NodeRef::DefElem(n) => n.location,
        NodeRef::RangeTblEntry(n) => todo!("RangeTblEntry"),
        NodeRef::RangeTblFunction(n) => todo!("RangeTblFunction"),
        NodeRef::TableSampleClause(n) => todo!("TableSampleClause"),
        NodeRef::WithCheckOption(n) => todo!("WithCheckOption"),
        NodeRef::SortGroupClause(n) => todo!("SortGroupClause"),
        NodeRef::GroupingSet(n) => n.location,
        NodeRef::WindowClause(n) => todo!("WindowClause"),
        NodeRef::ObjectWithArgs(n) => todo!("ObjectWithArgs"),
        NodeRef::AccessPriv(n) => todo!("AccessPriv"),
        NodeRef::CreateOpClassItem(n) => todo!("CreateOpClassItem"),
        NodeRef::TableLikeClause(n) => todo!("TableLikeClause"),
        NodeRef::FunctionParameter(n) => todo!("FunctionParameter"),
        NodeRef::LockingClause(n) => todo!("LockingClause"),
        NodeRef::RowMarkClause(n) => todo!("RowMarkClause"),
        NodeRef::XmlSerialize(n) => n.location,
        NodeRef::WithClause(n) => n.location,
        NodeRef::InferClause(n) => n.location,
        NodeRef::OnConflictClause(n) => n.location,
        NodeRef::CommonTableExpr(n) => n.location,
        NodeRef::RoleSpec(n) => n.location,
        NodeRef::TriggerTransition(n) => todo!("TriggerTransition"),
        NodeRef::PartitionElem(n) => n.location,
        NodeRef::PartitionSpec(n) => n.location,
        NodeRef::PartitionBoundSpec(n) => n.location,
        NodeRef::PartitionRangeDatum(n) => n.location,
        NodeRef::PartitionCmd(n) => todo!("PartitionCmd"),
        NodeRef::VacuumRelation(n) => todo!("VacuumRelation"),
        NodeRef::InlineCodeBlock(n) => todo!("InlineCodeBlock"),
        NodeRef::CallContext(n) => todo!("CallContext"),
        NodeRef::Integer(n) => todo!("Integer"),
        NodeRef::Float(n) => todo!("Float"),
        NodeRef::String(n) => todo!("String"),
        NodeRef::BitString(n) => todo!("BitString"),
        NodeRef::Null(n) => todo!("Null"),
        NodeRef::List(n) => todo!("List"),
        NodeRef::IntList(n) => todo!("IntList"),
        NodeRef::OidList(n) => todo!("OidList"),
        _ => -1,
    }
}

#[cfg(test)]
mod tests {
    use std::assert_eq;
    use std::fs;
    use std::path::Path;

    use super::*;

    #[test]
    fn test_get_flat_nodes() {
        let input = "CREATE TABLE products (
    product_no integer,
    name text,
    price numeric CHECK (price > 0),
    discounted_price numeric CHECK (discounted_price > 0),
    CHECK (price > discounted_price)
);";

        let parsed = pg_query::parse(input);
    }
}
