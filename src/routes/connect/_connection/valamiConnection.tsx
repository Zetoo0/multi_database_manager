import { createFileRoute, useNavigate } from '@tanstack/react-router'
import { useHotkey } from '../../../hooks/hotkey/useHotkey'
import { useLayout } from '../../../providers/LayoutProvider'
import { useEffect } from 'react'
import { ConnectionForm } from '../../../components/forms/ConnectionForm'
import { ConnectionSubMenu } from '../../../components/connection/ConnectionSubMenu'
import { ConnectionSubGroup } from '../../../components/connection/ConnectionSubGroup'
import { useTranslation } from 'react-i18next'

export const Route = createFileRoute('/connect/_connection/valamiConnection')({
  component: ConnectionPage
})


function ConnectionPage() {
    try {
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
        <ConnectionSubMenu title={t("connection.newConnection")}>
          <ConnectionSubGroup title={t("connection.connect")}>
            <ConnectionForm
                onConnection={() => {console.log('Connection');navigate({ to: '/' });}}
                onMetadatasGet={() => console.log('Metadatas')}
              />
          </ConnectionSubGroup>
        </ConnectionSubMenu>
        /*
        <SplitLayout
          left={<ConnectionMenu />}
          right={
            <ConnectionForm
              onConnection={() => {console.log('Connection');navigate({ to: '/' });}}
              onMetadatasGet={() => console.log('Metadatas')}
            />
          }
        />*/
      )
    } catch (error) {
      console.error('Error in ConnectionPage:', error)
      return <div>Error loading ConnectionPage</div>
    }
  }
  