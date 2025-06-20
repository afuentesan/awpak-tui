import type { DataFrom } from "./data";

export type DataComparator = DataComparatorEq | 
                             DataComparatorNotEq | 
                             DataComparatorGt | 
                             DataComparatorLt | 
                             DataComparatorRegex |
                             DataComparatorAnd |
                             DataComparatorOr |
                             DataComparatorNot |
                             DataComparatorTrue |
                             DataComparatorFalse;

export class DataComparatorEq
{
    _variant = "Eq";

    from_1 : DataFrom | undefined;
    from_2 : DataFrom | undefined;
}

export class DataComparatorNotEq
{
    _variant = "NotEq";

    from_1 : DataFrom | undefined;
    from_2 : DataFrom | undefined;
}

export class DataComparatorGt
{
    _variant = "Gt";

    from_1 : DataFrom | undefined;
    from_2 : DataFrom | undefined;
}

export class DataComparatorLt
{
    _variant = "Lt";

    from_1 : DataFrom | undefined;
    from_2 : DataFrom | undefined;
}

export class DataComparatorRegex
{
    _variant = "Regex";

    regex : string | undefined;
    from : DataFrom | undefined;
}

export class DataComparatorAnd
{
    _variant = "And";

    comp_1 : DataComparator | undefined;
    comp_2 : DataComparator | undefined;
}

export class DataComparatorOr
{
    _variant = "Or";

    comp_1 : DataComparator | undefined;
    comp_2 : DataComparator | undefined;
}

export class DataComparatorNot
{
    _variant = "Not";

    value : DataComparator | undefined;
}

export class DataComparatorTrue
{
    _variant = "True";
}

export class DataComparatorFalse
{
    _variant = "False";
}
