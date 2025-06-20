
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
}


export enum DataMerge
{
    Insert = "Insert",
    Append = "Append"
}

export class DataToString
{
    from : DataFrom | undefined;
    prefix : string | undefined;
    suffix : string | undefined;
}

export type DataFrom = FromContext | FromParsedInput | FromInput | FromStatic | FromConcat | FromOperation;

export class FromContext
{
    _variant = "Context";

    path : string | undefined;
    required : boolean | undefined;
}

export class FromParsedInput
{
    _variant = "ParsedInput";

    path : string | undefined;
    required : boolean | undefined;
}

export class FromStatic
{
    _variant = "Static";

    value : any
}

export class FromInput
{
    _variant = "Input";

    required : boolean | undefined;
}

export class FromOperation
{
    _variant  = "Operation";

    value : DataOperation | undefined;
}

export class FromConcat
{
    _variant = "Concat";

    value : Array<DataFrom> = [];
}

export type DataOperation = DataOperationLen | DataOperationSubstract | DataOperationAdd;

export class DataOperationLen
{
    _variant = "Len";

    value : DataFrom | undefined;
}

export class DataOperationSubstract
{
    _variant = "Substract";

    num_1 : DataFrom | undefined;
    num_2 : DataFrom | undefined;
}

export class DataOperationAdd
{
    _variant = "Add";

    num_1 : DataFrom | undefined;
    num_2 : DataFrom | undefined;
}