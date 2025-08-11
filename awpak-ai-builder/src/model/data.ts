
export enum DataType
{
    Null = "Null",
    Bool = "Bool",
    Number = "Number",
    String = "String",
    Array = "Array",
    Object = "Object"
}

export class DataToContext
{
    path : string | undefined;
    ty : DataType | undefined;
    merge : DataMerge | undefined;
    optional : boolean = false;
}


export enum DataMerge
{
    Insert = "Insert",
    Append = "Append",
    AppendToArray = "AppendToArray"
}

export class DataToString
{
    from : DataFrom | undefined;
    prefix : string | undefined;
    suffix : string | undefined;

    constructor()
    {
        this.from = new FromContext();
    }
}

export type DataFrom = FromContext | 
                       FromParsedInput | 
                       FromInput | 
                       FromStatic | 
                       FromConcat | 
                       FromOperation | 
                       FromNull | 
                       FromAgentHistory |
                       FromStore;

export enum DataFromVariant
{
    Context = "Context",
    ParsedInput = "ParsedInput",
    Static = "Static",
    Input = "Input",
    Operation = "Operation",
    Concat = "Concat",
    Null = "Null",
    AgentHistory = "AgentHistory",
    Store = "Store"
}

export class FromStore
{
    readonly _variant = DataFromVariant.Store;

    id : string = "";
    query : DataFrom = new FromStatic();
    samples : number = 1;
}

export class FromContext
{
    readonly _variant = DataFromVariant.Context;

    path : string | undefined;
    required : boolean | undefined;
}

export class FromParsedInput
{
    readonly _variant = DataFromVariant.ParsedInput;

    path : string | undefined;
    required : boolean | undefined;
}

export class FromStatic
{
    readonly _variant = DataFromVariant.Static;

    value : any = ""
}

export class FromInput
{
    readonly _variant = DataFromVariant.Input;

    required : boolean | undefined;
}

export class FromOperation
{
    readonly _variant  = DataFromVariant.Operation;

    value : DataOperation | undefined;

    constructor()
    {
        this.value = new DataOperationLen();
    }
}

export class FromConcat
{
    readonly _variant = DataFromVariant.Concat;

    value : Array<DataFrom> = [];
}

export class FromNull
{
    readonly _variant = DataFromVariant.Null;
}

export class FromAgentHistory
{
    readonly _variant = DataFromVariant.AgentHistory;

    id : string = "";
    content : FromAgentHistoryContent = new FromAgentHistoryContentFull();
}

export type FromAgentHistoryContent = FromAgentHistoryContentFull |
                                      FromAgentHistoryContentFullMessages |
                                      FromAgentHistoryContentFirst |
                                      FromAgentHistoryContentFirstMessage |
                                      FromAgentHistoryContentLast |
                                      FromAgentHistoryContentLastMessage |
                                      FromAgentHistoryContentRange |
                                      FromAgentHistoryContentRangeMessages |
                                      FromAgentHistoryContentItem |
                                      FromAgentHistoryContentItemMessage;

export enum FromAgentHistoryContentVariant
{
    Full = "Full",
    FullMessages = "FullMessages",

    First = "First",
    FirstMessage = "FirstMessage",

    Last = "Last",
    LastMessage = "LastMessage",

    Range = "Range",
    RangeMessages = "RangeMessages",

    Item = "Item",
    ItemMessage = "ItemMessage"
}

export class FromAgentHistoryContentFull
{
    readonly _variant = FromAgentHistoryContentVariant.Full;
}

export class FromAgentHistoryContentFullMessages
{
    readonly _variant = FromAgentHistoryContentVariant.FullMessages;
}

export class FromAgentHistoryContentFirst
{
    readonly _variant = FromAgentHistoryContentVariant.First;
}

export class FromAgentHistoryContentFirstMessage
{
    readonly _variant = FromAgentHistoryContentVariant.FirstMessage;
}

export class FromAgentHistoryContentLast
{
    readonly _variant = FromAgentHistoryContentVariant.Last;
}

export class FromAgentHistoryContentLastMessage
{
    readonly _variant = FromAgentHistoryContentVariant.LastMessage;
}

export class FromAgentHistoryContentRange
{
    readonly _variant = FromAgentHistoryContentVariant.Range;

    from : number = 0;
    to : number = 0;
}

export class FromAgentHistoryContentRangeMessages
{
    readonly _variant = FromAgentHistoryContentVariant.RangeMessages;

    from : number = 0;
    to : number = 0;
}

export class FromAgentHistoryContentItem
{
    readonly _variant = FromAgentHistoryContentVariant.Item;

    value : number = 0;
}

export class FromAgentHistoryContentItemMessage
{
    readonly _variant = FromAgentHistoryContentVariant.ItemMessage;

    value : number = 0;
}

export type DataOperation = DataOperationLen | DataOperationSubstract | DataOperationAdd | DataOperationStringSplit;

export enum DataOperationVariant
{
    Len = "Len",
    Substract = "Substract",
    Add = "Add",
    StringSplit = "StringSplit"
}

export class DataOperationLen
{
    readonly _variant = DataOperationVariant.Len;

    value : DataFrom | undefined;

    constructor()
    {
        this.value = new FromContext();
    }
}

export class DataOperationSubstract
{
    readonly _variant = DataOperationVariant.Substract;

    num_1 : DataFrom | undefined;
    num_2 : DataFrom | undefined;

    constructor()
    {
        this.num_1 = new FromContext();
        this.num_2 = new FromContext();
    }
}

export class DataOperationAdd
{
    readonly _variant = DataOperationVariant.Add;

    num_1 : DataFrom | undefined;
    num_2 : DataFrom | undefined;

    constructor()
    {
        this.num_1 = new FromContext();
        this.num_2 = new FromContext();
    }
}

export class DataOperationStringSplit
{
    readonly _variant = DataOperationVariant.StringSplit;

    from : DataFrom = new FromContext();
    sep : string = "";
}