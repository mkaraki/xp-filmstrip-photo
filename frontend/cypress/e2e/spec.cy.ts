describe('Film Strip Viewer', () => {
  it('loads the root directory and shows folders', () => {
    cy.visit('/')
    cy.get('.sidebar').should('be.visible')
    cy.get('.tree-node').should('contain', 'Sample01')
    cy.get('.tree-node').should('contain', 'Sample02')
  })

  it('navigates to a folder and shows images', () => {
    cy.visit('/Sample01')
    cy.get('.view-header').should('contain', 'Sample01')
    cy.get('.thumb-wrapper').should('have.length.at.least', 1)
    cy.get('.large-image').should('be.visible')
  })

  it('changes selected image on click', () => {
    // Add another image to Sample01 for this test
    cy.visit('/Sample01')
    cy.get('.thumb-wrapper').first().click()
    cy.get('.thumb-wrapper').first().should('have.class', 'active')
  })
})
