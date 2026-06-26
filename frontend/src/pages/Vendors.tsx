import {useEffect, useState} from "react";
import type {Vendor} from "@/models/vendor.ts";
import axios from "axios";
import {BASE_URL} from "@/main.tsx";
import {Table, TableBody, TableCaption, TableCell, TableHead, TableHeader, TableRow} from "@/components/ui/table.tsx";
import {
    DropdownMenu,
    DropdownMenuContent,
    DropdownMenuItem,
    DropdownMenuTrigger
} from "@/components/ui/dropdown-menu.tsx";
import {Button} from "@/components/ui/button.tsx";
import {CheckIcon, MoreHorizontalIcon} from "lucide-react";
import {Input} from "@/components/ui/input.tsx";
import {Field, FieldGroup} from "@/components/ui/field.tsx";
import {toast} from "sonner";
import {
    Dialog, DialogClose,
    DialogContent,
    DialogDescription,
    DialogFooter,
    DialogHeader,
    DialogTitle
} from "@/components/ui/dialog.tsx";
import {Label} from "@/components/ui/label.tsx";

export default function Vendors() {
    const [vendors, setVendors] = useState<Vendor[]>([])

    // inputs and stuff
    const [addVendorValue, setAddVendorValue] = useState<string | undefined>(undefined)
    const [editVendorValue, setEditVendorValue] = useState<string | undefined>(undefined)
    const [editingVendor, setEditingVendor] = useState<Vendor | undefined>(undefined)

    function isVendorNameValid(name: string | undefined): boolean {
        if (name === undefined || name.trim() === "") {
            return false
        }
        return true

    }

    // add
    const addVendor = () => {
        if (!isVendorNameValid(addVendorValue) || addVendorValue === "a") {
            toast.error("New Vendor name is invalid")
            return
        }

        axios.post<Vendor>(`${BASE_URL}/vendor`, {"name": addVendorValue})
            .then(resp => {
                setVendors(
                    prev => prev ? [...prev, resp.data] : prev
                )
                setAddVendorValue("")
                toast.success(`Vendor \"${resp.data.name}\" was successfully added`);
            })
            .catch(e => {
                console.error(e)
                toast.error(`Error returned from backend: ${e}`)
            })
    }

    // delete
    function deleteVendor(id: number) {
        axios.delete<Vendor>(`${BASE_URL}/vendor/${id}`)
            .then(resp => {
                setVendors(
                    prev => prev.filter(v => v.id !== id)
                )
                toast.success(`Vendor \"${resp.data.name}\" successfully deleted`)
            })
            .catch(e => {
                console.error(e)
                toast.error(`Error returned from backend: ${e}`)
            })
    }

    // edit
    function editVendor(vendor: Vendor, newName: string) {
        if (!isVendorNameValid(editVendorValue)) {
            toast.error("Edited Vendor name is invalid")
            return
        }
        axios.patch<Vendor>(`${BASE_URL}/vendor/${vendor.id}`, {"name": newName})
            .then(() => {
                const oldName = vendor.name
                vendor.name = newName
                setEditingVendor(undefined)
                setEditVendorValue(undefined)
                toast.success(`Vendor "${oldName}" successfully updated to \"${newName}\"`)
            })
            .catch(e =>  {
                console.error(e)
                toast.error(`Error returned from backend: ${e}`)
            })
    }

    // page load
    useEffect(() => {
        axios.get<Vendor[]>(`${BASE_URL}/vendor`)
            .then(resp => setVendors(resp.data))
            // .then(resp => console.log(resp.data))
            .catch(e => console.error(e))
    }, []);

    return (
        <main>
            <h2>Vendors</h2>
            <Table>
                <TableCaption>Total: {vendors.length} vendor{vendors.length > 1 && 's'}</TableCaption>
                <TableHeader>
                    <TableRow>
                        <TableHead>Vendor Name</TableHead>
                        <TableHead className={"w-32 text-right"}>Actions</TableHead>
                    </TableRow>
                </TableHeader>
                <TableBody>
                    {
                        vendors.map(v => (
                            <TableRow>
                                <TableCell>{v.name}</TableCell>
                                <TableCell className={"text-right"}>
                                    <DropdownMenu>
                                        <DropdownMenuTrigger asChild>
                                            <Button variant={"ghost"} size={"icon-lg"}>
                                                <MoreHorizontalIcon/>
                                            </Button>
                                        </DropdownMenuTrigger>
                                        <DropdownMenuContent>
                                            {/* edit and delete */}
                                            <DropdownMenuItem
                                                onClick={() => setEditingVendor(v)}
                                            >
                                                Edit
                                            </DropdownMenuItem>
                                            <DropdownMenuItem
                                                variant={"destructive"}
                                                onClick={() => deleteVendor(v.id)}
                                            >
                                                Delete
                                            </DropdownMenuItem>
                                        </DropdownMenuContent>
                                    </DropdownMenu>
                                </TableCell>
                            </TableRow>
                        ))
                    }
                    <TableRow>
                        <TableCell colSpan={2}>
                            <Field orientation={"horizontal"}>
                                <Input
                                    placeholder={"Add new vendor..."}
                                    className={"w-96"}
                                    value={addVendorValue}
                                    onChange={e => setAddVendorValue(e.target.value)}
                                />
                                {
                                    isVendorNameValid(addVendorValue) && <Button variant={"secondary"} onClick={addVendor}>
                                        <CheckIcon/>
                                    </Button>
                                }
                            </Field>
                        </TableCell>
                    </TableRow>
                </TableBody>
            </Table>
            {/* edit */}
            {
                editingVendor !== undefined && <Dialog open={true}>
                    <DialogContent>
                        <DialogHeader>
                            <DialogTitle>Edit vendor</DialogTitle>
                            <DialogDescription>Change the name of "{editingVendor.name}"</DialogDescription>
                        </DialogHeader>
                        <FieldGroup>
                            <Field>
                                <Label>New name</Label>
                                <Input
                                    value={editVendorValue}
                                    onChange={e => setEditVendorValue(e.target.value)}
                                    defaultValue={editingVendor.name}
                                />
                            </Field>
                        </FieldGroup>
                        <DialogFooter>
                            <DialogClose asChild>
                                <Button variant={"outline"} onClick={() => setEditingVendor(undefined)}>Cancel</Button>
                            </DialogClose>
                            <Button variant={"secondary"} onClick={() => editVendor(editingVendor, editVendorValue!)}>Edit name</Button>
                        </DialogFooter>
                    </DialogContent>
            </Dialog>
            }
        </main>
    )
}