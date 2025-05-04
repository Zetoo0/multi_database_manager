import { createFileRoute, useNavigate } from "@tanstack/react-router";
import { ConnectionSubMenu} from "../../../components/connection/ConnectionSubMenu";
import { ConnectionSubGroup } from "../../../components/connection/ConnectionSubGroup";
import { ConnectionList } from "../../../components/connection/connectionList/ConnectionList";
import { useEffect } from "react";
import { useHotkey } from "../../../hooks/hotkey/useHotkey";
import { useLayout } from "../../../providers/LayoutProvider";
import { useTranslation } from "react-i18next";
export const Route = createFileRoute('/connect/_connection/connections')({
    component: ConnectionListPage
})


function ConnectionListPage() {

    const navigate = useNavigate()
    const { setShowStatusBar, setShowTitleBar } = useLayout()
    const { /*formatted*/ } = useHotkey('closeSettings', () => {
      navigate({ to: '/' })
    })
    const {t} = useTranslation("form");
    useEffect(() => {
      setShowStatusBar(false)
      setShowTitleBar(false)
      return () => {
        setShowStatusBar(true)
        setShowTitleBar(true)
      }
    }, [])
    return (
        <ConnectionSubMenu title={t("connection.connections")}>
            <ConnectionSubGroup title={t("connection.connectionList")}>
                <ConnectionList onConnection={() => navigate({ to: '/' })} />
            </ConnectionSubGroup>
        </ConnectionSubMenu>
    )
}


