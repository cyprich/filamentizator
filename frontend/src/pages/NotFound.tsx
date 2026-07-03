import {Typography} from "@heroui/react";
import {HeartCrackIcon} from "lucide-react";
import {Link} from "react-router";

export default function NotFound() {
    return (
        <main className={"flex flex-col items-center justify-center -mt-12 mb-12"}>
            <HeartCrackIcon className={"size-16"}/>
            <Typography type={"h1"} className={"mb-4"}>Not Found</Typography>
            <p>This page was not found</p>
            <p>Check if the link is correct, or {" "}
                <Link to={"/"} className={"clickable-no-scale underline"}>return home</Link>
            </p>
        </main>
    )
}