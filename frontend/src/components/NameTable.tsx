import {AlertDialog, Button, Input, Label, Pagination, Table, TextField} from "@heroui/react";
import {useEffect, useMemo, useState} from "react";
import type {Vendor} from "../types/vendor.ts";
import type {Material} from "../types/material.ts";
import {PencilIcon, TrashIcon} from "lucide-react";
import axios from "axios";
import {BASE_URL} from "../main.tsx";

const ROWS_PER_PAGE = 10;
const columns = [
    {id: "name", name: "Name"},
    {id: "actions", name: "Actions"}
]

export type GeneralTableProps = {
    data: Vendor[] | Material[];
    dataName: "Vendor" | "Material";
    paginationSize: number;
    className?: string;
}

export function NameTable(props: GeneralTableProps) {
    const [data, setData] = useState<Vendor[] | Material[]>([])
    const [editItem, setEditItem] = useState<undefined | Vendor | Material>(undefined)
    const [newItemName, setNewItemName] = useState<undefined | string>(undefined)
    const [deleteItem, setDeleteItem] = useState<undefined | Vendor | Material>(undefined)

    const [page, setPage] = useState(1);
    const totalPages = Math.ceil(data.length / ROWS_PER_PAGE);
    // const pages = Array.from({length: totalPages}, (_, i) => i + 1);

    const paginatedItems = useMemo(() => {
        const start = (page - 1) * ROWS_PER_PAGE;

        return data.slice(start, start + ROWS_PER_PAGE);
    }, [page, data]);

    const start = data.length === 0 ? 0 : (page - 1) * ROWS_PER_PAGE + 1;
    const end = Math.min(page * ROWS_PER_PAGE, data.length);

    const editClicked = () => {
        if (editItem === undefined || newItemName === undefined) {
            cancelEverything()
            return
        }

        const endpoint = props.dataName.toLowerCase()
        axios.patch(`${BASE_URL}/${endpoint}/${editItem.id}`, {name: newItemName})
            .then(resp => {
                console.log(resp)
                // TODO toast
                setData(old => old.map(i => i.id === editItem.id ? {...i, name: newItemName} : i))
                cancelEverything()
            })
            .catch(e => console.error(e))
    }

    const deleteClicked = () => {
        if (deleteItem === undefined) {
            cancelEverything()
            return
        }

        const endpoint = props.dataName.toLowerCase()
        axios.delete(`${BASE_URL}/${endpoint}/${deleteItem.id}`)
            .then(resp => {
                console.log(resp)
                setData(old => old.filter(i => i.id !== deleteItem.id))
                cancelEverything()
            })
            .catch(e => {
                console.error(e)
            })
    }

    const cancelEverything = () => {
        setEditItem(undefined)
        setDeleteItem(undefined)
        setNewItemName(undefined)
    }

    useEffect(() => {
        setData(props.data)
    }, [props.data]);

    return (
        <>
            <Table className={props.className}>
                <Table.ScrollContainer>
                    <Table.Content>

                        {/* table header */}
                        <Table.Header columns={columns}>
                            <Table.Column isRowHeader={true}>Name</Table.Column>
                            <Table.Column className={"w-48 text-right"}>Actions</Table.Column>
                        </Table.Header>

                        {/* table body */}
                        <Table.Body items={paginatedItems}>
                            {
                                paginatedItems.map(item => (
                                    <Table.Row>
                                        <Table.Cell>{item.name}</Table.Cell>
                                        <Table.Cell className={"flex gap-2 justify-end *:size-5"}>
                                            {/* edit button */}
                                            <PencilIcon
                                                className={"clickable"}
                                                onClick={() => setEditItem(item)}
                                            />
                                            {/* delete button */}
                                            <TrashIcon
                                                className={"clickable hover:text-danger"}
                                                onClick={() => {
                                                    setDeleteItem(item)
                                                    setNewItemName(item.name)
                                                }}
                                            />
                                        </Table.Cell>
                                    </Table.Row>
                                ))
                            }
                        </Table.Body>
                    </Table.Content>
                </Table.ScrollContainer>

                {/* table footer */}
                <Table.Footer>
                    <Pagination>
                        <Pagination.Summary>
                            {start} to {end} of {data.length} items
                        </Pagination.Summary>
                        <Pagination.Content>
                            <Pagination.Item>
                                <Pagination.Previous
                                    isDisabled={page === 1}
                                    onPress={() => setPage(p => Math.max(1, p-1))}
                                >
                                    <Pagination.PreviousIcon/>
                                </Pagination.Previous>
                                <Pagination.Next
                                    isDisabled={page === totalPages}
                                    onPress={() => setPage(p => Math.min(totalPages, p+1))}
                                >
                                    <Pagination.NextIcon/>
                                </Pagination.Next>
                            </Pagination.Item>
                        </Pagination.Content>
                    </Pagination>
                </Table.Footer>
            </Table>
            {/* edit dialog */}
            <AlertDialog isOpen={editItem !== undefined}>
                <AlertDialog.Backdrop>
                    <AlertDialog.Container>
                        <AlertDialog.Dialog>
                            <AlertDialog.CloseTrigger onClick={cancelEverything}/>
                            <AlertDialog.Header>
                                <AlertDialog.Icon status={"accent"}/>
                                <AlertDialog.Heading>Edit {props.dataName}</AlertDialog.Heading>
                            </AlertDialog.Header>
                            <AlertDialog.Body>
                                <TextField name={"editName"}>
                                    <Label>New name for <strong>{editItem?.name}</strong></Label>
                                    <Input
                                        value={newItemName}
                                        onChange={(e) => setNewItemName(e.target.value)}
                                        placeholder={"Enter new name..."}
                                    />
                                </TextField>
                            </AlertDialog.Body>
                            <AlertDialog.Footer>
                                <Button
                                    slot={"close"}
                                    variant={"tertiary"}
                                    onClick={cancelEverything}
                                >Cancel</Button>
                                <Button
                                    variant={"primary"}
                                    onClick={editClicked}
                                >Edit</Button>
                            </AlertDialog.Footer>
                        </AlertDialog.Dialog>
                    </AlertDialog.Container>
                </AlertDialog.Backdrop>
            </AlertDialog>

            {/* delete dialog */}
            <AlertDialog isOpen={deleteItem !== undefined}>
                <AlertDialog.Backdrop>
                    <AlertDialog.Container>
                        <AlertDialog.Dialog>
                            <AlertDialog.CloseTrigger onClick={cancelEverything}/>
                            <AlertDialog.Header>
                                <AlertDialog.Icon status={"danger"}/>
                                <AlertDialog.Heading>Delete {props.dataName}?</AlertDialog.Heading>
                            </AlertDialog.Header>
                            <AlertDialog.Body>
                                <p>Are you sure you want to delete <strong>{deleteItem?.name}</strong>?</p>
                                <p>This action cannot be undone.</p>
                            </AlertDialog.Body>
                            <AlertDialog.Footer>
                                <Button
                                    slot={"close"}
                                    variant={"tertiary"}
                                    onClick={cancelEverything}
                                >Cancel</Button>
                                <Button
                                    variant={"danger"}
                                    onClick={deleteClicked}
                                >Delete</Button>
                            </AlertDialog.Footer>
                        </AlertDialog.Dialog>
                    </AlertDialog.Container>
                </AlertDialog.Backdrop>
            </AlertDialog>
        </>
    )
}
