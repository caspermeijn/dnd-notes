export const useApi = (path, options = {}) => {
    return useFetch(`https://dnd-notes-backend.herokuapp.com/api/${path}`, options)
}