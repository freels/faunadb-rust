//! A special case of an expression that needs to be evaluated to a value.
pub mod auth;
pub mod basic;
pub mod collection;
pub mod conversion;
pub mod datetime;
pub mod logical;
pub mod math;
pub mod misc;
pub mod read;
pub mod set;
pub mod string;
pub mod write;

#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum Query<'a> {
    Difference(set::Difference<'a>),
    Distinct(set::Distinct<'a>),
    Intersection(set::Intersection<'a>),
    Join(set::Join<'a>),
    Match(set::Match<'a>),
    Union(set::Union<'a>),

    Date(datetime::Date<'a>),
    Epoch(datetime::Epoch<'a>),
    Time(datetime::Time<'a>),

    CaseFold(string::CaseFold<'a>),
    Concat(string::Concat<'a>),
    FindStr(string::FindStr<'a>),
    FindStrRegex(string::FindStrRegex<'a>),
    LTrim(string::LTrim<'a>),
    Length(string::Length<'a>),
    LowerCase(string::LowerCase<'a>),
    RTrim(string::RTrim<'a>),
    Repeat(string::Repeat<'a>),
    ReplaceStr(string::ReplaceStr<'a>),
    ReplaceStrRegex(string::ReplaceStrRegex<'a>),
    Space(string::Space<'a>),
    SubString(string::SubString<'a>),
    TitleCase(string::TitleCase<'a>),
    Trim(string::Trim<'a>),
    UpperCase(string::UpperCase<'a>),

    HasIdentity(auth::HasIdentity<'a>),
    Identify(auth::Identify<'a>),
    Identity(auth::Identity<'a>),
    Login(auth::Login<'a>),
    Logout(auth::Logout<'a>),

    ToDate(conversion::ToDate<'a>),
    ToNumber(conversion::ToNumber<'a>),
    ToString(conversion::ToString<'a>),
    ToTime(conversion::ToTime<'a>),

    At(basic::At<'a>),
    Call(basic::Call<'a>),
    Do(basic::Do<'a>),
    Let(basic::Let<'a>),
    Var(basic::Var<'a>),
    Lambda(basic::Lambda<'a>),
    If(basic::If<'a>),

    Append(collection::Append<'a>),
    Drop(collection::Drop<'a>),
    Filter(collection::Filter<'a>),
    Foreach(collection::Foreach<'a>),
    IsEmpty(collection::IsEmpty<'a>),
    IsNonEmpty(collection::IsNonEmpty<'a>),
    Map(collection::Map<'a>),
    Prepend(collection::Prepend<'a>),
    Take(collection::Take<'a>),

    And(logical::And<'a>),
    Or(logical::Or<'a>),
    Not(logical::Not<'a>),
    Lt(logical::Lt<'a>),
    Lte(logical::Lte<'a>),
    Gt(logical::Gt<'a>),
    Gte(logical::Gte<'a>),
    Contains(logical::Contains<'a>),
    Equals(logical::Equals<'a>),
    Exists(logical::Exists<'a>),

    Abs(math::Abs<'a>),
    Acos(math::Acos<'a>),
    Add(math::Add<'a>),
    Asin(math::Asin<'a>),
    Atan(math::Atan<'a>),
    BitAnd(math::BitAnd<'a>),
    BitNot(math::BitNot<'a>),
    BitOr(math::BitOr<'a>),
    BitXor(math::BitXor<'a>),
    Ceil(math::Ceil<'a>),
    Cos(math::Cos<'a>),
    Cosh(math::Cosh<'a>),
    Degrees(math::Degrees<'a>),
    Divide(math::Divide<'a>),
    Exp(math::Exp<'a>),
    Floor(math::Floor<'a>),
    Hypot(math::Hypot<'a>),
    Ln(math::Ln<'a>),
    Log(math::Log<'a>),
    Max(math::Max<'a>),
    Min(math::Min<'a>),
    Modulo(math::Modulo<'a>),
    Multiply(math::Multiply<'a>),
    Pow(math::Pow<'a>),
    Radians(math::Radians<'a>),
    Round(math::Round<'a>),
    Sign(math::Sign<'a>),
    Sin(math::Sin<'a>),
    Sinh(math::Sinh<'a>),
    Sqrt(math::Sqrt<'a>),
    Subtract(math::Subtract<'a>),
    Tan(math::Tan<'a>),
    Tanh(math::Tanh<'a>),
    Trunc(math::Trunc<'a>),

    CreateClass(Box<write::CreateClass<'a>>),
    CreateDatabase(write::CreateDatabase<'a>),
    CreateIndex(Box<write::CreateIndex<'a>>),
    CreateFunction(Box<write::CreateFunction<'a>>),
    CreateKey(Box<write::CreateKey<'a>>),
    Create(write::Create<'a>),
    Insert(Box<write::Insert<'a>>),
    Delete(write::Delete<'a>),
    Remove(write::Remove<'a>),
    Replace(write::Replace<'a>),
    Update(write::Update<'a>),

    Get(read::Get<'a>),
    KeyFromSecret(read::KeyFromSecret<'a>),
    Paginate(read::Paginate<'a>),
    Select(read::Select<'a>),
    SelectAll(read::SelectAll<'a>),

    Abort(misc::Abort<'a>),
    Class(misc::Class<'a>),
    Classes(misc::Classes<'a>),
    Database(misc::Database<'a>),
    Databases(misc::Databases<'a>),
    Function(misc::Function<'a>),
    Functions(misc::Functions<'a>),
    Index(misc::Index<'a>),
    Indexes(misc::Indexes<'a>),
    NewId(misc::NewId<'a>),
}
