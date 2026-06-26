import {Button} from "@/components/ui/button.tsx";
import {
    type LucideIcon,
    CircleUserIcon,
    EqualApproximatelyIcon, FactoryIcon, HomeIcon,
    SidebarIcon,
    SpoolIcon,
} from "lucide-react";
import {useState} from "react";
import {useNavigate} from "react-router";

type Section = {
    name: string;
    icon: LucideIcon;
    link: string;
}

export default function CustomSidebar() {
    const navigate = useNavigate();
    const [isOpen, setIsOpen] = useState<boolean>(true)

    const sections: Section[] = [
        { name: "Home", icon: HomeIcon, link: "/" },
        { name: "Filaments", icon: SpoolIcon, link: "/filaments" },
        { name: "Vendors", icon: FactoryIcon, link: "/vendors" },
        { name: "Materials", icon: EqualApproximatelyIcon, link: "/materials" },
    ]

    const toggleIsOpen = () => {
        setIsOpen(!isOpen)
    }

    return (
        <div className={"fixed top-0 left-0 z-10 flex flex-col justify-between items-start h-screen p-4 bg-neutral-100 border-r border-r-neutral-300 w-48"}>
            {/*top section*/}
            <div className={" flex flex-col gap-4 w-full"}>
                <Button variant={"secondary"} size={"icon-lg"} className={"self-end"} onClick={toggleIsOpen}>
                    <SidebarIcon className={"icon"}/>
                </Button>
                <div className={"flex flex-col gap-2 items-start"}>
                    {
                        sections.map(s => {
                            return (
                                <Button variant={"secondary"} onClick={() => navigate(s.link)} className={"wide-button"}>
                                    <s.icon className={"icon"} />
                                    {s.name}
                                </Button>
                            )
                        })
                    }
                </div>
            </div>
            {/*bottom section*/}
            <div>
                <Button variant={"secondary"} className={"w-full justify-start"} onClick={() => navigate("/account")}>
                    <CircleUserIcon className={"icon"}/>
                    Account
                </Button>
            </div>
        </div>
    )
}
