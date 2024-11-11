
def resized(page, containers: []):
    for reference in containers:
        reference.current.width = page.width
    page.update()