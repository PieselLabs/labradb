use sqlparser::ast;
use sqlparser::dialect::GenericDialect;
use sqlparser::parser::Parser;
use sqlparser::parser::ParserError;

#[derive(Debug)]
pub struct TableScan {
    pub table_name: String,
}

#[derive(Debug)]
pub struct Filter {
    pub input: LogicalPlan,
}

#[derive(Debug)]
pub struct Project {
    pub input: LogicalPlan,
    pub columns: Vec<String>,
}

#[derive(Debug)]
pub enum LogicalPlan {
    Scan(Box<TableScan>),
    Filter(Box<Filter>),
    Project(Box<Project>),
}

impl LogicalPlan {
    pub fn from_query(query: &str) -> Result<Self, ParserError> {
        let dialect = GenericDialect {};
        let statements = Parser::parse_sql(&dialect, query)?;

        assert!(statements.len() == 1);

        let statement = &statements[0];

        match statement {
            ast::Statement::Query(q) => Self::parse_query(q),

            _ => unimplemented!(),
        }
    }

    fn parse_query(query: &ast::Query) -> Result<Self, ParserError> {
        match *query.body {
            ast::SetExpr::Select(ref select) => Self::parse_select(&select),
            _ => unimplemented!(),
        }
    }

    fn parse_select(select: &ast::Select) -> Result<Self, ParserError> {
        assert!(select.from.len() == 1);
        let mut plan = Self::parse_from(&select.from[0])?;

        plan = Self::parse_projection(&select.projection, plan)?;

        if let Some(filter) = &select.selection {
            plan = Self::parse_where(&filter, plan)?;
        }

        Ok(plan)
    }

    fn parse_from(table: &ast::TableWithJoins) -> Result<Self, ParserError> {
        assert!(table.joins.len() == 0);
        match &table.relation {
            ast::TableFactor::Table { name, .. } => Ok(LogicalPlan::Scan(Box::new(TableScan {
                table_name: name.to_string(),
            }))),
            _ => unimplemented!(),
        }
    }

    fn parse_where(expr: &ast::Expr, input: LogicalPlan) -> Result<Self, ParserError> {
        todo!()
    }

    fn parse_projection(
        projection: &Vec<ast::SelectItem>,
        input: LogicalPlan,
    ) -> Result<Self, ParserError> {
        let columns = projection
            .iter()
            .map(|item| match item {
                ast::SelectItem::UnnamedExpr(expr) => match expr {
                    ast::Expr::Identifier(ident) => ident.value.clone(),
                    _ => unimplemented!(),
                },
                _ => unimplemented!(),
            })
            .collect();

        Ok(LogicalPlan::Project(Box::new(Project { input, columns })))
    }
}
