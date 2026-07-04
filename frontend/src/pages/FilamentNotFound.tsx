import { Typography } from "@heroui/react";
import GeneralNotFound from "./GeneralNotFound.tsx";
import {Link} from "react-router";

export type FilamentNotFoundProps = {
    id?: string | number | undefined
    cause: "notFound" | "invalidID"
}

export default function FilamentNotFound(props: FilamentNotFoundProps) {
    return (
        <GeneralNotFound title={"Filament Not Found"}>
            {
                props.cause === "invalidID" ? (
                    props.id
                        ? <p>ID <Typography type={"code"}>{props.id}</Typography> couldn't be converted to a number</p>
                        : <p>ID is <Typography type={"code"}>undefined</Typography></p>
                ) : <></>
            }
            {
                props.cause === "notFound" ? (
                    <p>
                        Filament {" "}
                        {
                            props.id ? <>with ID <Typography type={"code"}>{props.id}</Typography></> : <></>
                        }
                        {" "} was not found on the server
                    </p>
                ) : <></>
            }
            <p>Please check if the ID is correct, or go to the <Link to={"/filaments"}>Filaments</Link> page</p>
        </GeneralNotFound>
    )
}