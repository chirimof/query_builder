use super::dev::*;
use super::multiple_placeholder;

use std::borrow::Cow;


pub trait Condition: AsSqlParts
    where Self: Sized
{
    fn and<Cond: Condition> (self, cond: Cond) -> And<Self, Cond> {
        And::new(self, cond)
    }

    fn or<Cond: Condition> (self, cond: Cond) -> Or<Self, Cond> {
        Or::new(self, cond)
    }

    fn and_not<Cond: Condition> (self, cond: Cond) -> AndNot<Self, Cond> {
        AndNot::new(self, cond)
    }

    fn or_not<Cond: Condition> (self, cond: Cond) -> OrNot<Self, Cond> {
        OrNot::new(self, cond)
    }

    fn priority(self) -> Priority<Self> {
        Priority::new(self)
    }

}

pub struct Equal<Col> {
    column: Col
}

impl<Col> Equal<Col>
    where Col: AsColumn
{
    pub fn new(column: Col) -> Self {
        Equal { column }
    }
}

impl<Col> Condition for Equal<Col>
    where Col: AsColumn
{
}

impl<Col> AsSqlParts for Equal<Col>
    where Col: AsColumn
{
    fn as_sql_parts<'a> (&self) -> Cow<'a, str> {
        format!("{} = ?", self.column.as_sql_parts()).into()
    }
}

pub struct NotEq<Col>
{
    column: Col
}

impl<Col> NotEq<Col>
    where Col: AsColumn
{
    pub fn new(column: Col) -> Self {
        NotEq { column }
    }
}

impl<Col> Condition for NotEq<Col>
    where Col: AsColumn
{
}

impl<Col> AsSqlParts for NotEq<Col>
    where Col: AsColumn
{

    fn as_sql_parts<'a> (&self) -> Cow<'a, str> {
        format!("{} != ?", self.column.as_sql_parts()).into()
    }
}

pub struct Greater<Col>
{
    column: Col
}

impl<Col> Greater<Col>
    where Col: AsColumn
{
    pub fn new(column: Col) -> Self {
        Greater { column }
    }
}

impl<Col> Condition for Greater<Col>
    where Col: AsColumn
{
}

impl<Col> AsSqlParts for Greater<Col>
    where Col: AsColumn
{
    fn as_sql_parts<'a> (&self) -> Cow<'a, str> {
        format!("{} > ?", self.column.as_sql_parts()).into()
    }
}

pub struct GreaterEq<Col> {
    column: Col
}

impl<Col> GreaterEq<Col>
    where Col: AsColumn
{
    pub fn new(column: Col) -> Self {
        GreaterEq { column }
    }
}

impl<Col> Condition for GreaterEq<Col>
    where Col: AsColumn
{}

impl<Col> AsSqlParts for GreaterEq<Col>
    where Col: AsColumn
{
    fn as_sql_parts<'a> (&self) -> Cow<'a, str> {
        format!("{} >= ?", self.column.as_sql_parts()).into()
    }
}

pub struct Less<Col> {
    column: Col
}

impl<Col> Less<Col>
    where Col: AsColumn
{
    pub fn new(column: Col) -> Self {
        Less { column }
    }
}

impl<Col> Condition for Less<Col>
    where Col: AsColumn
{
}

impl<Col> AsSqlParts for Less<Col>
    where Col: AsColumn
{
    fn as_sql_parts<'a> (&self) -> Cow<'a, str> {
        format!("{} < ?", self.column.as_sql_parts()).into()
    }
}

pub struct LessEq<Col> {
    column: Col
}

impl<Col> LessEq<Col>
    where Col: AsColumn
{
    pub fn new(column: Col) -> Self {
        LessEq { column }
    }
}

impl<Col> Condition for LessEq<Col>
    where Col: AsColumn
{
}

impl<Col> AsSqlParts for LessEq<Col>
    where Col: AsColumn
{
    fn as_sql_parts<'a> (&self) -> Cow<'a, str> {
        format!("{} <= ?", self.column.as_sql_parts()).into()
    }
}

pub struct Between<Col> {
    column: Col
}

impl<Col> Between<Col>
    where Col: AsColumn
{
    pub fn new(column: Col) -> Self {
        Between { column }
    }
}

impl<Col> Condition for Between<Col>
    where Col: AsColumn
{}

impl<Col> AsSqlParts for Between<Col>
    where Col: AsColumn
{
    fn as_sql_parts<'a> (&self) -> Cow<'a, str> {
        format!("{} BETWEEN ? AND ?", self.column.as_sql_parts()).into()
    }
}

pub struct Included<Col> {
    column: Col,
    len: usize
}

impl<Col> Included<Col>
    where Col: AsColumn
{
    pub fn new(column: Col, len: usize) -> Self {
        Included { column, len }
    }
}

impl<Col> Condition for Included<Col>
    where Col: AsColumn
{
}

impl<Col> AsSqlParts for Included<Col>
    where Col: AsColumn
{
    fn as_sql_parts<'a> (&self) -> Cow<'a, str> {
        format!("{} IN ( {} )", self.column.as_sql_parts(), multiple_placeholder(self.len, "?")).into()
    }
}

pub struct Like<Col> {
    column: Col
}

impl<Col> Like<Col>
    where Col: AsColumn
{
    pub fn new(column: Col) -> Self {
        Like { column }
    }
}

impl<Col> Condition for Like<Col>
    where Col: AsColumn
{
}

impl<Col> AsSqlParts for Like<Col>
    where Col: AsColumn
{
    fn as_sql_parts<'a> (&self) -> Cow<'a, str> {
        format!("{} LIKE ?", self.column.as_sql_parts()).into()
    }
}

// Wrap two Condition ==========
pub struct And<L, R>
{
    left: L,
    right: R,
}

impl<L, R> And<L, R>
    where
        L: Condition,
        R: Condition,
{
    fn new(left: L, right: R) -> Self {
        And { left, right }
    }
}

impl<L, R> Condition for And<L, R>
    where
        L: Condition,
        R: Condition
{
}

impl<L, R> AsSqlParts for And<L, R>
    where
        L: Condition,
        R: Condition
{
    fn as_sql_parts<'a> (&self) -> Cow<'a, str> {
        format!("{} AND {}", self.left.as_sql_parts(), self.right.as_sql_parts()).into()
    }
}

pub struct Or<L, R>
{
    left: L,
    right: R,
}

impl<L, R> Or<L, R>
    where
        L: Condition,
        R: Condition
{
    pub fn new(left: L, right: R) -> Self {
        Or { left, right }
    }
}

impl<L, R> Condition for Or<L, R>
    where
        L: Condition,
        R: Condition
{
}

impl<L, R> AsSqlParts for Or<L, R>
    where
        L: Condition,
        R: Condition
{
    fn as_sql_parts<'a> (&self) -> Cow<'a, str> {
        format!("{} OR {}", self.left.as_sql_parts(), self.right.as_sql_parts()).into()
    }
}

pub struct AndNot<L, R> {
    left: L,
    right: R,
}

impl<L, R> AndNot<L, R>
    where
        L: Condition,
        R: Condition
{
    pub fn new(left: L, right: R) -> Self {
        AndNot { left, right }
    }
}

impl<L, R> Condition for AndNot<L, R>
    where
        L: Condition,
        R: Condition
{}

impl<L, R> AsSqlParts for AndNot<L, R>
    where
        L: Condition,
        R: Condition
{
    fn as_sql_parts<'a> (&self) -> Cow<'a, str> {
        format!("{} AND NOT ( {} )", self.left.as_sql_parts(), self.right.as_sql_parts()).into()
    }
}

pub struct OrNot<L, R> {
    left: L,
    right: R,
}

impl<L, R> OrNot<L, R>
    where
        L: Condition,
        R: Condition
{
    pub fn new(left: L, right: R) -> Self {
        OrNot { left, right }
    }
}

impl<L, R> Condition for OrNot<L, R>
    where
        L: Condition,
        R: Condition
{}

impl<L, R> AsSqlParts for OrNot<L, R>
    where
        L: Condition,
        R: Condition
{
    fn as_sql_parts<'a> (&self) -> Cow<'a, str> {
        format!("{} OR NOT ( {} )", self.left.as_sql_parts(), self.right.as_sql_parts()).into()
    }
}

pub struct Priority<Cond>
{
    condition: Cond
}

impl<Cond> Priority<Cond>
    where Cond: Condition
{
    pub fn new(condition: Cond) -> Self {
        Priority { condition }
    }
}

impl<Cond> Condition for Priority<Cond>
    where Cond: Condition
{
}

impl<Cond> AsSqlParts for Priority<Cond>
    where Cond: Condition
{
    fn as_sql_parts<'a> (&self) -> Cow<'a, str> {
        format!("( {} )", self.condition.as_sql_parts()).into()
    }
}

#[cfg(test)]
mod simple_test {
    #![allow(dead_code)]
    use super::*;

    setup_table!({
        namespace: users,
        column_set: {id: Id, admin: Admin},
    });
    use self::users::*;

    #[test]
    fn eq_test() {
        let parts = Id.equal();
        assert_eq!(parts.as_sql_parts(), "users.id = ?");
    }

    #[test]
    fn not_eq_test() {
        let parts = Id.not_eq();
        assert_eq!(parts.as_sql_parts(), "users.id != ?");
    }

    #[test]
    fn greater_test() {
        let parts = Id.greater();
        assert_eq!(parts.as_sql_parts(), "users.id > ?");
    }

    #[test]
    fn greater_eq_test() {
        let parts = Id.greater_eq();
        assert_eq!(parts.as_sql_parts(), "users.id >= ?");
    }

    #[test]
    fn less_test() {
        let parts = Id.less();
        assert_eq!(parts.as_sql_parts(), "users.id < ?");
    }

    #[test]
    fn less_eq_test() {
        let parts = Id.less_eq();
        assert_eq!(parts.as_sql_parts(), "users.id <= ?");
    }

    #[test]
    fn between_test() {
        let parts = Id.between();
        assert_eq!(parts.as_sql_parts(), "users.id BETWEEN ? AND ?");
    }

    #[test]
    fn included_test() {
        let parts = Id.included(3);
        assert_eq!(parts.as_sql_parts(), "users.id IN ( ?, ?, ? )");
    }

    #[test]
    fn and_test() {
        let parts = Id.equal().and(Admin.not_eq());
        assert_eq!(parts.as_sql_parts(), "users.id = ? AND users.admin != ?");
    }

    #[test]
    fn or_test() {
        let parts = Id.equal().or(Admin.equal());
        assert_eq!(parts.as_sql_parts(), "users.id = ? OR users.admin = ?");
    }
}

#[cfg(test)]
mod complex_test {
    #![allow(dead_code)]
    use super::*;

    setup_table!({
        namespace: users,
        column_set: {id: Id, name: Name, email: Email, admin: Admin},
    });

    use self::users::*;

    #[test]
    fn level_1_test() {
        // A AND B OR C AND B
        let expected = "users.id = ? AND users.name LIKE ? OR users.email LIKE ? AND users.admin != ?";
        let parts = Id.equal().and(Name.like()).or(Email.like()).and(Admin.not_eq());
        assert_eq!(parts.as_sql_parts(), expected);

        // same as above
        let parts = Id.equal().and(Name.like().or(Email.like()).and(Admin.not_eq()));
        assert_eq!(parts.as_sql_parts(), expected);
    }

    #[test]
    fn level_2_test() {
        // (A AND B) OR (C AND D)
        let expected =
            "( users.id = ? AND users.name LIKE ? ) OR ( users.email LIKE ? AND users.admin != ? )";

        let parts = Id.equal().and(Name.like()).priority().or(Email.like().and(Admin.not_eq()).priority());
        assert_eq!(parts.as_sql_parts(), expected);

        // Divide
        let expected = "( users.id = ? AND users.name LIKE ? )";
        let parts = Id.equal().and(Name.like()).priority();
        assert_eq!(parts.as_sql_parts(), expected);

        let expected = "users.id = ? AND ( users.name LIKE ? )";
        let parts = Id.equal().and(Name.like().priority());
        assert_eq!(parts.as_sql_parts(), expected);
    }

    #[test]
    fn level_3_test() {
        // A AND ( (B AND C) OR D )
        let expected =
            "users.admin = ? \
            OR ( \
                ( users.name = ? AND users.email = ? ) OR users.id IN ( ?, ?, ? ) \
            )";

        let parts = Admin.equal()
            .or(
                Name.equal().and(Email.equal()).priority()
                .or(Id.included(3))
                .priority()
            );
        assert_eq!(parts.as_sql_parts(), expected);
    }
}