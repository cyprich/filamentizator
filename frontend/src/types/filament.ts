import type {Material} from "./material.ts";
import type {Vendor} from "./vendor.ts";
import type {Temp} from "./temp.ts";
import type {Weight} from "./weight.ts";

export type Filament = {
    id: number;
    material: Material;
    vendor: Vendor;
    name: string;
    temp: Temp;
    weight: Weight;
    price: number;
    date_created: string;
    date_updated: string;
}