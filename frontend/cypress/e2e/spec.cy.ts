/// <reference types="cypress" />

type ExplorerItem = {
  name: string
  path: string
  is_dir: boolean
  mime: string | null
  has_subdirs: boolean
  size: number
  modified: number
}

describe('Filmstrip Photo Viewer E2E', () => {
  const now = 1735689600
  const tinyPngBase64 =
    'iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAQAAAC1HAwCAAAAC0lEQVR42mP8/x8AAwMCAO5mXv0AAAAASUVORK5CYII='

  const rootDirs: ExplorerItem[] = [
    {
      name: 'Web',
      path: 'Web',
      is_dir: true,
      mime: null,
      has_subdirs: true,
      size: 0,
      modified: now,
    },
    {
      name: 'ScreenShot',
      path: 'ScreenShot',
      is_dir: true,
      mime: null,
      has_subdirs: true,
      size: 0,
      modified: now,
    },
  ]

  const rootItems: ExplorerItem[] = [
    {
      name: 'cover.jpg',
      path: 'cover.jpg',
      is_dir: false,
      mime: 'image/jpeg',
      has_subdirs: false,
      size: 123456,
      modified: now,
    },
    ...rootDirs,
  ]

  const webItems: ExplorerItem[] = [
    {
      name: 'img100.jpg',
      path: 'Web/img100.jpg',
      is_dir: false,
      mime: 'image/jpeg',
      has_subdirs: false,
      size: 42000,
      modified: now,
    },
    {
      name: 'img101.jpg',
      path: 'Web/img101.jpg',
      is_dir: false,
      mime: 'image/jpeg',
      has_subdirs: false,
      size: 43000,
      modified: now,
    },
    {
      name: 'Wallpaper',
      path: 'Web/Wallpaper',
      is_dir: true,
      mime: null,
      has_subdirs: true,
      size: 0,
      modified: now,
    },
  ]

  const webDirs: ExplorerItem[] = [
    {
      name: 'Wallpaper',
      path: 'Web/Wallpaper',
      is_dir: true,
      mime: null,
      has_subdirs: false,
      size: 0,
      modified: now,
    },
  ]

  const wallpaperItems: ExplorerItem[] = [
    {
      name: 'wall-001.jpg',
      path: 'Web/Wallpaper/wall-001.jpg',
      is_dir: false,
      mime: 'image/jpeg',
      has_subdirs: false,
      size: 35000,
      modified: now,
    },
  ]

  const installHappyPathInterceptors = () => {
    cy.intercept('GET', '/.__api/dirs.json', { statusCode: 200, body: rootDirs }).as('dirsRoot')
    cy.intercept('GET', '/.__api/dirs/Web.json', { statusCode: 200, body: webDirs }).as('dirsWeb')
    cy.intercept('GET', '/.__api/dirs/Web/Wallpaper.json', { statusCode: 200, body: [] }).as('dirsWallpaper')

    cy.intercept('GET', '/.__api/list.json', { statusCode: 200, body: rootItems }).as('listRoot')
    cy.intercept('GET', '/.__api/list/Web.json', { statusCode: 200, body: webItems }).as('listWeb')
    cy.intercept('GET', '/.__api/list/Web/Wallpaper.json', { statusCode: 200, body: wallpaperItems }).as('listWallpaper')

    cy.intercept('GET', '/.__api/metadata/*.json', {
      statusCode: 200,
      body: { width: 1920, height: 1080 },
    }).as('metadata')

    cy.intercept('GET', '/.__api/thumb/**', {
      statusCode: 200,
      headers: { 'content-type': 'image/jpeg' },
      body: '',
    }).as('thumb')

    // Serve valid image bytes for full-size preview requests.
    cy.intercept({ method: 'GET', url: /\/[^?]+\.(jpg|jpeg|png)(\?.*)?$/i }, {
      statusCode: 200,
      headers: { 'content-type': 'image/png' },
      body: Cypress.Buffer.from(tinyPngBase64, 'base64'),
    }).as('imgAny')

    cy.intercept('GET', '/.__api/version.json', {
      statusCode: 200,
      body: {
        version: '9.9.9-test',
        build_time: '2026.0223.120000',
      },
    }).as('version')

    cy.intercept('POST', '/.__api/auth/logout', { statusCode: 200, body: '' }).as('logout')
  }

  beforeEach(() => {
    cy.clearCookies()
    cy.clearLocalStorage()
  })

  describe('Explorer Navigation', () => {
    it('loads root explorer layout, initial preview, and metadata', () => {
      installHappyPathInterceptors()

      cy.visit('/')

      cy.wait('@dirsRoot')
      cy.wait('@listRoot')
      cy.wait('@metadata')

      cy.get('.sidebar').should('be.visible')
      cy.get('.tree-node').should('have.length.at.least', 1)
      cy.get('.thumb-wrapper').should('have.length', rootItems.length)
      cy.get('.thumb-wrapper.active').should('have.length', 1)
      cy.get('.preview-container', { timeout: 10000 }).should(($container) => {
        const hasLargeImage = $container.find('.large-image').length > 0
        const hasNoPreview = $container.find('.no-preview').length > 0
        expect(hasLargeImage || hasNoPreview).to.eq(true)
      })
    })

    it('updates active selection and shows no preview when selecting a directory item', () => {
      installHappyPathInterceptors()

      cy.visit('/Web')
      cy.wait('@listWeb')

      cy.contains('.thumb-wrapper .thumb-name', 'Wallpaper').click()
      cy.get('.thumb-wrapper.active .thumb-name').should('contain.text', 'Wallpaper')
      cy.wait(600)
      cy.get('.no-preview').should('be.visible')
    })

    it('navigates into a folder by double-clicking a directory in filmstrip', () => {
      installHappyPathInterceptors()

      cy.visit('/')
      cy.wait('@listRoot')

      cy.contains('.thumb-wrapper .thumb-name', 'Web').dblclick()

      cy.wait('@listWeb')
      cy.location('pathname').should('eq', '/Web')
      cy.contains('.thumb-wrapper .thumb-name', 'img100.jpg').should('be.visible')
    })

    it('navigates using folder tree labels', () => {
      installHappyPathInterceptors()

      cy.visit('/')
      cy.wait('@dirsRoot')

      cy.contains('.node-label .label-text', 'Web').click()

      cy.wait('@listWeb')
      cy.location('pathname').should('eq', '/Web')
      cy.get('.thumb-wrapper').should('have.length', webItems.length)
    })

    it('navigates using address bar Enter', () => {
      installHappyPathInterceptors()

      cy.visit('/')
      cy.wait('@listRoot')

      cy.get('.address-input').clear().type('/Web{enter}')

      cy.wait('@listWeb')
      cy.location('pathname').should('eq', '/Web')
    })

    it('supports toolbar Back/Forward/Up navigation', () => {
      installHappyPathInterceptors()

      cy.visit('/Web')
      cy.wait('@listWeb')

      cy.contains('.thumb-wrapper .thumb-name', 'Wallpaper').dblclick()
      cy.wait('@listWallpaper')
      cy.location('pathname').should('eq', '/Web/Wallpaper')

      cy.get('button[title="Up"]').should('not.be.disabled').click()
      cy.wait('@listWeb')
      cy.location('pathname').should('eq', '/Web')

      cy.get('button[title="Back"]').should('not.be.disabled').click()
      cy.wait('@listWallpaper')
      cy.location('pathname').should('eq', '/Web/Wallpaper')

      cy.get('button[title="Forward"]').should('not.be.disabled').click()
      cy.wait('@listWeb')
      cy.location('pathname').should('eq', '/Web')
    })
  })

  describe('View Modes And Pane State', () => {
    it('switches view modes and persists selected mode after reload', () => {
      installHappyPathInterceptors()

      cy.visit('/Web')
      cy.wait('@listWeb')

      cy.get('.view-mode-toggle').click()
      cy.contains('.view-dropdown .dropdown-item', 'Thumbnails').click()
      cy.get('.thumbnails-view').should('be.visible')
      cy.get('.thumb-item').should('have.length', webItems.length)

      cy.get('.view-mode-toggle').click()
      cy.contains('.view-dropdown .dropdown-item', 'Details').click()
      cy.get('.details-view').should('be.visible')
      cy.get('.details-table tbody tr').should('have.length', webItems.length)

      cy.reload()
      cy.wait('@listWeb')
      cy.get('.details-view').should('be.visible')
      cy.window().then((win) => {
        expect(win.localStorage.getItem('xp-view-mode')).to.eq('details')
      })
    })

    it('supports keyboard navigation in details view', () => {
      installHappyPathInterceptors()

      cy.visit('/Web')
      cy.wait('@listWeb')

      cy.get('.view-mode-toggle').click()
      cy.contains('.view-dropdown .dropdown-item', 'Details').click()

      cy.get('.details-view').focus().trigger('keydown', { key: 'End' })
      cy.get('.details-table tbody tr.active .row-text').should('contain.text', 'Wallpaper')

      cy.get('.details-view').trigger('keydown', { key: 'Home' })
      cy.get('.details-table tbody tr.active .row-text').should('contain.text', 'img100.jpg')
    })

    it('toggles folders/task pane and persists sidebar mode', () => {
      installHappyPathInterceptors()

      cy.visit('/Web')
      cy.wait('@listWeb')

      cy.get('.folders-toggle').click()
      cy.get('.task-pane').should('be.visible')
      cy.window().then((win) => {
        expect(win.localStorage.getItem('xp-show-folders')).to.eq('false')
      })

      cy.reload()
      cy.wait('@listWeb')
      cy.get('.task-pane').should('be.visible')
    })

    it('rotates selected image in filmstrip preview', () => {
      installHappyPathInterceptors()

      cy.visit('/Web')
      cy.wait('@listWeb')

      cy.get('.tool-btn-small[title="Rotate Clockwise"]').click()
      cy.get('.preview-container', { timeout: 10000 }).then(($container) => {
        const $img = $container.find('.large-image')
        if ($img.length > 0) {
          expect(($img.attr('style') || '')).to.contain('rotate(90deg)')
        } else {
          expect($container.find('.no-preview').length > 0).to.eq(true)
        }
      })

      cy.get('.tool-btn-small[title="Rotate Counterclockwise"]').click()
      cy.get('.preview-container').then(($container) => {
        const $img = $container.find('.large-image')
        if ($img.length > 0) {
          expect(($img.attr('style') || '')).to.contain('rotate(0deg)')
        } else {
          expect($container.find('.no-preview').length > 0).to.eq(true)
        }
      })
    })
  })

  describe('Slideshow And Menus', () => {
    it('starts slideshow from task pane and supports next/prev/escape controls', () => {
      installHappyPathInterceptors()

      cy.visit('/Web')
      cy.wait('@listWeb')

      cy.get('.folders-toggle').click()
      cy.contains('.task-item', 'View as a slide show').click()

      cy.wait('@listWeb')
      cy.location('pathname').should('eq', '/.__slideshow/Web')
      cy.get('.slideshow-overlay').should('be.visible')
      cy.get('.slide-image').should('have.attr', 'src').and('contain', '/Web/img100.jpg')

      cy.get('.xp-slideshow-toolbar button[title="Next"]').click()
      cy.get('.slide-image').should('have.attr', 'src').and('contain', '/Web/img101.jpg')

      cy.get('.xp-slideshow-toolbar button[title="Previous"]').click()
      cy.get('.slide-image').should('have.attr', 'src').and('contain', '/Web/img100.jpg')

      cy.get('.xp-slideshow-toolbar button[title="Pause"]').click()
      cy.get('.xp-slideshow-toolbar button[title="Play"]').click()

      cy.get('.slideshow-overlay').focus().trigger('keydown', { key: 'Escape' })
      cy.location('pathname').should('eq', '/Web')
      cy.get('.slideshow-overlay').should('not.exist')
    })

    it('opens About popup from Help menu', () => {
      installHappyPathInterceptors()

      cy.visit('/', {
        onBeforeLoad(win) {
          cy.stub(win, 'open').as('windowOpen')
        },
      })

      cy.wait('@listRoot')
      cy.contains('.menu-item', 'Help').click()
      cy.contains('.xp-menu-dropdown .dropdown-item', 'About this tool').click()

      cy.get('@windowOpen').should('have.been.calledWithMatch', '/.__ui/about')
    })
  })

  describe('Auth And System Routes', () => {
    it('opens login dialog when API returns 401', () => {
      cy.intercept('GET', '/.__api/dirs.json', { statusCode: 401, body: [] }).as('dirs401')
      cy.intercept('GET', '/.__api/list.json', { statusCode: 401, body: [] }).as('list401')

      cy.visit('/', {
        onBeforeLoad(win) {
          cy.stub(win, 'open').as('openStub')
        },
      })

      cy.wait('@dirs401')
      cy.wait('@list401')
      cy.get('@openStub').should('have.been.calledWithMatch', '/.__ui/login')
    })

    it('submits login form and stores credentials when remember is enabled', () => {
      cy.intercept('POST', '/.__api/auth/login', (req) => {
        expect(req.headers.authorization).to.eq('Basic dXNlcjpwYXNz')
        req.reply({ statusCode: 200, body: '' })
      }).as('authLogin')

      cy.visit('/.__ui/login', {
        onBeforeLoad(win) {
          win.localStorage.clear()
          cy.stub(win, 'close').as('closeStub')
        },
      })

      cy.get('#username').clear().type('user')
      cy.get('#password').clear().type('pass')
      cy.get('button').contains('OK').click()

      cy.wait('@authLogin')
      cy.window().then((win) => {
        expect(win.localStorage.getItem('filmstrip_auth')).to.eq('dXNlcjpwYXNz')
      })
      cy.get('@closeStub').should('have.been.called')
    })

    it('submits login form without remember and does not persist to localStorage', () => {
      cy.intercept('POST', '/.__api/auth/login', { statusCode: 200, body: '' }).as('authLogin')

      cy.visit('/.__ui/login', {
        onBeforeLoad(win) {
          win.localStorage.clear()
          cy.stub(win, 'close').as('closeStub')
        },
      })

      cy.get('#username').clear().type('temp-user')
      cy.get('#password').clear().type('temp-pass')
      cy.get('#remember').uncheck()
      cy.get('button').contains('OK').click()

      cy.wait('@authLogin')
      cy.window().then((win) => {
        expect(win.localStorage.getItem('filmstrip_auth')).to.be.null
      })
      cy.get('@closeStub').should('have.been.called')
    })

    it('closes login dialog when cancel is pressed', () => {
      cy.visit('/.__ui/login', {
        onBeforeLoad(win) {
          cy.stub(win, 'close').as('closeStub')
        },
      })

      cy.get('button').contains('Cancel').click()
      cy.get('@closeStub').should('have.been.calledOnce')
    })

    it('disconnects network drive from Tools menu and calls logout endpoint', () => {
      installHappyPathInterceptors()

      cy.visit('/', {
        onBeforeLoad(win) {
          win.localStorage.setItem('filmstrip_auth', 'dGVzdDp0ZXN0')
          cy.stub(win, 'confirm').as('confirmStub').returns(true)
        },
      })

      cy.wait('@listRoot')
      cy.contains('.menu-item', 'Tools').click()
      cy.contains('.xp-menu-dropdown .dropdown-item', 'Disconnect Network Drive...').click()

      cy.get('@confirmStub').should('have.been.calledOnce')
      cy.wait('@logout')
      cy.window().then((win) => {
        expect(win.localStorage.getItem('filmstrip_auth')).to.be.null
      })
    })

    it('loads About page and fetches backend version', () => {
      installHappyPathInterceptors()

      cy.visit('/.__ui/about')

      cy.wait('@version')
      cy.contains('Web based File Explorer').should('be.visible')
      cy.contains('Backend v9.9.9-test').should('be.visible')
      cy.get('button').contains('OK').should('be.visible')
    })
  })
})
