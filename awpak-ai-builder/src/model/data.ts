
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

export type DataFrom = FromContext | FromParsedInput | FromInput | FromStatic | FromConcat | FromOperation | FromNull;

export enum DataFromVariant
{
    Context = "Context",
    ParsedInput = "ParsedInput",
    Static = "Static",
    Input = "Input",
    Operation = "Operation",
    Concat = "Concat",
    Null = "Null"
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

    value : any
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

export type DataOperation = DataOperationLen | DataOperationSubstract | DataOperationAdd;

export enum DataOperationVariant
{
    Len = "Len",
    Substract = "Substract",
    Add = "Add"
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