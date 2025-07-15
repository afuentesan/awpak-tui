import { FromStatic, type DataFrom, type DataToContext } from "./data";
import { DataComparatorTrue, type DataComparator } from "./data_comparator";

export class ContextMut
{
    from : DataFrom = new FromStatic();
    to : DataToContext | undefined;
    condition : DataComparator = new DataComparatorTrue();
}