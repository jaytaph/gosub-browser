// These are the states in which the tokenizer can be in.
#[derive(Debug)]
pub enum State {
    DataState,
    CharacterReferenceInDataState,
    RcDataState,
    CharacterReferenceInRcDataState,
    RawTextState,
    ScriptDataState,
    PlaintextState,
    TagOpenState,
    EndTagOpenState,
    TagNameState,
    RcDataLessThanSignState,
    RcDataEndTagOpenState,
    RcDataEndTagNameState,
    RawTextLessThanSignState,
    RawTextEndTagOpenState,
    RawTextEndTagNameState,
    ScriptDataLessThenSignState,
    ScriptDataEndTagOpenState,
    ScriptDataEndTagNameState,
    ScriptDataEscapeStartState,
    ScriptDataEscapeStartDashState,
    ScriptDataEscapedState,
    ScriptDataEscapedDashState,
    ScriptDataEscapedLessThanSignState,
    ScriptDataEscapedEndTagOpenState,
    ScriptDataEscapedEndTagNameState,
    ScriptDataDoubleEscapeStartState,
    ScriptDataDoubleEscapedState,
    ScriptDataDoubleEscapedDashState,
    ScriptDataDoubleEscapedDashDashState,
    ScriptDataDoubleEscapedLessThanSignState,
    ScriptDataDoubleEscapeEndState,
    BeforeAttributeNameState,
    AttributeNameState,
    BeforeAttributeValueState,
    AttributeValueDoubleQuotedState,
    AttributeValueSingleQuotedState,
    AttributeValueUnquotedState,
    CharacterReferenceInAttributeValueState,
    AfterAttributeValueQuotedState,
    SelfClosingStartState,
    BogusCommentState,
    MarkupDeclarationOpenState,
    CommentStartState,
    CommentStartDashState,
    CommentState,
    CommentEndDashState,
    CommentEndState,
    CommentEndBangState,
    DocTypeState,
    BeforeDocTypeNameState,
    DocTypeNameState,
    AfterDocTypeNameState,
    AfterDocTypePublicKeywordState,
    BeforeDocTypePublicIdentifierState,
    DocTypePublicIdentifierDoubleQuotedState,
    DocTypePublicIdentifierSingleQuotedState,
    AfterDoctypePublicIdentifierState,
    BetweenDocTypePublicAndSystemIdentifiersState,
    AfterDocTypeSystemKeywordState,
    BeforeDocTypeSystemIdentifiedState,
    DocTypeSystemIdentifierDoubleQuotedState,
    DocTypeSystemIdentifierSingleQuotedState,
    AfterDocTypeSystemIdentifiedState,
    BogusDocTypeState,
    CDataSectionState,
}
