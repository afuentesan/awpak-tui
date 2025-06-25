import { FromContext, type DataFrom } from "./data";

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

export enum DataComparatorVariant
{
    Eq = "Eq",
    NotEq = "NotEq",
    Gt = "Gt",
    Lt = "Lt",
    Regex = "Regex",
    And = "And",
    Or = "Or",
    Not = "Not",
    True = "True",
    False = "False"
}

export class DataComparatorEq
{
    readonly _variant = DataComparatorVariant.Eq;

    from_1 : DataFrom | undefined;
    from_2 : DataFrom | undefined;

    constructor()
    {
        this.from_1 = new FromContext();
        this.from_2 = new FromContext();
    }
}

export class DataComparatorNotEq
{
    readonly _variant = DataComparatorVariant.NotEq;

    from_1 : DataFrom | undefined;
    from_2 : DataFrom | undefined;

    constructor()
    {
        this.from_1 = new FromContext();
        this.from_2 = new FromContext();
    }
}

export class DataComparatorGt
{
    readonly _variant = DataComparatorVariant.Gt;

    from_1 : DataFrom | undefined;
    from_2 : DataFrom | undefined;

    constructor()
    {
        this.from_1 = new FromContext();
        this.from_2 = new FromContext();
    }
}

export class DataComparatorLt
{
    readonly _variant = DataComparatorVariant.Lt;

    from_1 : DataFrom | undefined;
    from_2 : DataFrom | undefined;

    constructor()
    {
        this.from_1 = new FromContext();
        this.from_2 = new FromContext();
    }
}

export class DataComparatorRegex
{
    readonly _variant = DataComparatorVariant.Regex;

    regex : string | undefined;
    from : DataFrom | undefined;

    constructor()
    {
        this.regex = undefined;
        this.from = new FromContext();
    }
}

export class DataComparatorAnd
{
    readonly _variant = DataComparatorVariant.And;

    comp_1 : DataComparator | undefined;
    comp_2 : DataComparator | undefined;

    constructor()
    {
        this.comp_1 = new DataComparatorEq();
        this.comp_2 = new DataComparatorEq();
    }
}

export class DataComparatorOr
{
    readonly _variant = DataComparatorVariant.Or;

    comp_1 : DataComparator | undefined;
    comp_2 : DataComparator | undefined;

    constructor()
    {
        this.comp_1 = new DataComparatorEq();
        this.comp_2 = new DataComparatorEq();
    }
}

export class DataComparatorNot
{
    readonly _variant = DataComparatorVariant.Not;

    value : DataComparator | undefined;

    constructor()
    {
        this.value = new DataComparatorEq();
    }
}

export class DataComparatorTrue
{
    readonly _variant = DataComparatorVariant.True;
}

export class DataComparatorFalse
{
    readonly _variant = DataComparatorVariant.False;
}
