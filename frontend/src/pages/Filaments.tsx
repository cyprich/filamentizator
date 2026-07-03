import {Table, TableBody, TableCaption, TableCell, TableHead, TableHeader, TableRow} from "@/components/ui/table.tsx";
import {useEffect, useState} from "react";
import axios from "axios";
import type {Filament} from "@/models/filament.ts";
import {BASE_URL} from "@/main.tsx";
import {toast} from "sonner";
import type {Vendor} from "@/models/vendor.ts";
import type {Material} from "@/models/material.ts";
import {Select, SelectContent, SelectItem, SelectTrigger, SelectValue} from "@/components/ui/select.tsx";

export default function Filaments() {
    const [filaments, setFilaments] = useState<Filament[]>([])
    const [vendors, setVendors] = useState<Vendor[]>([])
    const [materials, setMaterials] = useState<Material[]>([])

    useEffect(() => {
        axios.get<Filament[]>(`${BASE_URL}/filament`)
            .then(resp => {setFilaments(resp.data)})
            .catch(e => {
                console.error(e)
                toast.error(`Backend returned error while requesting filaments: ${e}`)
            })

        axios.get<Vendor[]>(`${BASE_URL}/vendor`)
            .then(resp => {setVendors(resp.data)})
            .catch(e => {
                console.error(e)
                toast.error(`Backend returned error while requesting vendors: ${e}`)
            })

        axios.get<Material[]>(`${BASE_URL}/material`)
            .then(resp => {setMaterials(resp.data)})
            .catch(e => {
                console.error(e)
                toast.error(`Backend returned error while requesting materials: ${e}`)
            })
    }, []);

    return (
        <main>
            <h2>Filaments</h2>
            <Table>
                <TableCaption>Total: {filaments.length} filament{filaments.length > 1 && 's'}</TableCaption>
                <TableHeader>
                    <TableRow>
                        <TableHead>Name</TableHead>
                        <TableHead>Vendor</TableHead>
                        <TableHead>Material</TableHead>
                        <TableHead>Nozzle Temperature</TableHead>
                        <TableHead>Bed Temperature</TableHead>
                        <TableHead>Price</TableHead>
                    </TableRow>
                </TableHeader>
                <TableBody>
                    {
                        filaments.map(f => (
                            <TableRow>
                                <TableCell>{f.name}</TableCell>
                                <TableCell>
                                    <Select defaultValue={f.vendor.name}>
                                        <SelectTrigger>
                                            <SelectValue placeholder={"Select Vendor"}/>
                                        </SelectTrigger>
                                        <SelectContent>
                                            {
                                                vendors.map(v => (
                                                    <SelectItem value={v.name}>{v.name}</SelectItem>
                                                ))
                                            }
                                        </SelectContent>
                                    </Select>
                                </TableCell>
                                <TableCell>
                                    <Select defaultValue={f.material.name}>
                                        <SelectTrigger>
                                            <SelectValue placeholder={"Select Material"}/>
                                        </SelectTrigger>
                                        <SelectContent>
                                            {
                                                materials.map(m => (
                                                    <SelectItem value={m.name}>{m.name}</SelectItem>
                                                ))
                                            }
                                        </SelectContent>
                                    </Select>
                                </TableCell>
                                <TableCell>
                                    {f.temp_min}
                                    {f.temp_max && `- ${f.temp_max}`} °C
                                </TableCell>
                                <TableCell>
                                    {f.temp_bed_min}
                                    {f.temp_bed_max && `- ${f.temp_bed_max}`} °C
                                </TableCell>
                                <TableCell>{f.price} €</TableCell>
                            </TableRow>
                        ))
                    }
                </TableBody>
            </Table>
        </main>
    )
}