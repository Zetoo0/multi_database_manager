import {
  createFileRoute,
  Link,
  Outlet,
  useNavigate,
} from '@tanstack/react-router'
import { useLayout } from '../../providers/LayoutProvider'
import { useHotkey } from '../../hooks/hotkey/useHotkey'
import { useEffect } from 'react'

import { SplitLayout } from '../../components/layout/PageLayout/SplitLayout'
import { X } from 'lucide-react'
import { Button } from '../../components/ui/Button'
import { MigrationMenu } from '../../components/migration/MigrationMenu'
import { useTranslation } from 'react-i18next'

export const Route = createFileRoute('/migration/_migration')({
  component: MigrationPage ,
})


function MigrationPage(){
    const navigate = useNavigate()
    const {t} = useTranslation("common")
    const { setShowStatusBar, setShowTitleBar } = useLayout()
    const { /*formatted*/ } = useHotkey('closeSettings', () => {
      navigate({ to: '/' })
    })

    

    useEffect(() => {
      console.log("What the hell is happening? ");
      setShowStatusBar(false)
      setShowTitleBar(false)
      return () => {
        setShowStatusBar(true)
        setShowTitleBar(true)
      }
    }, [])
    return(
        <SplitLayout left={<MigrationMenu/>}
        right={
          <div className='grid grid-cols-[auto_1fr] gap-2'>
            <Outlet />
            <Button
              variant="outline"
              className="sticky top size-8 rounded-full border-2 border-muted-foreground/70 p-0 text-muted-foreground/70 hover:border-muted-foreground"
              asChild
            >
              <Link to="/">
                <X strokeWidth="3" className="size-4" />
                <span className="absolute inset-x top-full mx-auto mt-2 w-min text-xs uppercase">
                  {t("actions.close")}
                </span>
              </Link>
            </Button>
          </div>
        } leftSize='20%'
        />
    )
}