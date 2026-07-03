import {Link} from "react-router";
import {HomeIcon, SpoolIcon, UserRoundIcon} from "lucide-react";

export default function Navbar() {
    return (
        <nav className={"p-2 w-max flex gap-4 *:px-4 py-2 rounded-xl fixed top-4 left-1/2 -translate-x-1/2 border border-zinc-300"}>
            <Link to={"/"}>
                <HomeIcon/>
                Home
            </Link>
            <Link to={"/filaments"}>
                <SpoolIcon/>
                Filaments
            </Link>
            <Link to={"/account"}>
                <UserRoundIcon/>
                Account
            </Link>
        </nav>
    )
}